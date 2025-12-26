use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub fn check_crate(tcx: TyCtxt<'_>) {
    let _prof_timer = tcx.sess.timer("type_check_crate");
    tcx.sess.time("coherence_checking", || {
        type R = Result<(), ErrorGuaranteed>;
        let _: R = tcx.ensure_ok().check_type_wf(());
        for &trait_def_id in tcx.all_local_trait_impls(()).keys() {
            let _: R = tcx.ensure_ok().coherent_trait(trait_def_id);
        }
        let _: R = tcx.ensure_ok().crate_inherent_impls_validity_check(());
        let _: R = tcx.ensure_ok().crate_inherent_impls_overlap_check(());
    });
    tcx.sess.time("emit_ast_lowering_delayed_lints", || {
        #[cfg(debug_assertions)]
        {
            for owner_id in tcx.hir_crate_items(()).owners() {
                if let Some(delayed_lints) = tcx.opt_ast_lowering_delayed_lints(owner_id) {
                    if !delayed_lints.lints.is_empty() {
                        assert!(tcx
                            .hir_crate_items(())
                            .delayed_lint_items()
                            .any(|i| i == owner_id));
                    }
                }
            }
        }
        for owner_id in tcx.hir_crate_items(()).delayed_lint_items() {
            if let Some(delayed_lints) = tcx.opt_ast_lowering_delayed_lints(owner_id) {
                for lint in &delayed_lints.lints {
                    emit_delayed_lint(lint, tcx);
                }
            }
        }
    });
    tcx.par_hir_body_owners(|item_def_id| {
        let def_kind = tcx.def_kind(item_def_id);
        match def_kind {
            DefKind::Static { .. } => {
                tcx.ensure_ok().eval_static_initializer(item_def_id);
                check::maybe_check_static_with_link_section(tcx, item_def_id);
            }
            DefKind::Const
                if !tcx.generics_of(item_def_id).own_requires_monomorphization()
                    && !find_attr!(tcx.get_all_attrs(item_def_id), AttributeKind::TypeConst(_)) =>
            {
                let instance = ty::Instance::new_raw(item_def_id.into(), ty::GenericArgs::empty());
                let cid = GlobalId {
                    instance,
                    promoted: None,
                };
                let typing_env = ty::TypingEnv::fully_monomorphized();
                tcx.ensure_ok()
                    .eval_to_const_value_raw(typing_env.as_query_input(cid));
            }
            _ => {}
        }
        if !(matches!(def_kind, DefKind::AnonConst) || def_kind.is_typeck_child()) {
            tcx.ensure_ok().typeck(item_def_id);
        }
        if tcx.needs_coroutine_by_move_body_def_id(item_def_id.to_def_id()) {
            tcx.ensure_done().coroutine_by_move_body_def_id(item_def_id);
        }
    });
    if tcx.features().rustc_attrs() {
        tcx.sess.time("dumping_rustc_attr_data", || {
            outlives::dump::inferred_outlives(tcx);
            variance::dump::variances(tcx);
            collect::dump::opaque_hidden_types(tcx);
            collect::dump::predicates_and_item_bounds(tcx);
            collect::dump::def_parents(tcx);
            collect::dump::vtables(tcx);
        });
    }
    tcx.ensure_ok().check_unused_traits(());
}
