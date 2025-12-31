use crate::rustc_ast::attr;
use crate::rustc_hir::def_id::LocalDefId;
use crate::rustc_middle::query::Providers;
use crate::rustc_middle::ty::TyCtxt;
use crate::rustc_span::sym;

fn proc_macro_decls_static(tcx: TyCtxt<'_>, (): ()) -> Option<LocalDefId> {
    let mut decls = None;

    for id in tcx.hir_free_items() {
        let attrs = tcx.hir_attrs(id.hir_id());
        if attr::contains_name(attrs, sym::rustc_proc_macro_decls) {
            decls = Some(id.owner_id.def_id);
        }
    }

    decls
}

pub(crate) fn provide(providers: &mut Providers) {
    *providers = Providers { proc_macro_decls_static, ..*providers };
}