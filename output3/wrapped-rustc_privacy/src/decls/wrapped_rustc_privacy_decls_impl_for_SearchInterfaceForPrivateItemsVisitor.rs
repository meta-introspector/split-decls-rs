use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl SearchInterfaceForPrivateItemsVisitor<'_> {
    fn generics(&mut self) -> &mut Self {
        self.in_primary_interface = true;
        for param in &self.tcx.generics_of(self.item_def_id).own_params {
            match param.kind {
                GenericParamDefKind::Lifetime => {}
                GenericParamDefKind::Type { has_default, .. } => {
                    if has_default {
                        let _ = self.visit(self.tcx.type_of(param.def_id).instantiate_identity());
                    }
                }
                GenericParamDefKind::Const { .. } => {
                    let _ = self.visit(self.tcx.type_of(param.def_id).instantiate_identity());
                }
            }
        }
        self
    }
    fn predicates(&mut self) -> &mut Self {
        self.in_primary_interface = false;
        let _ = self.visit_predicates(self.tcx.explicit_predicates_of(self.item_def_id));
        self
    }
    fn bounds(&mut self) -> &mut Self {
        self.in_primary_interface = false;
        let _ = self.visit_clauses(
            self.tcx
                .explicit_item_bounds(self.item_def_id)
                .skip_binder(),
        );
        self
    }
    fn ty(&mut self) -> &mut Self {
        self.in_primary_interface = true;
        let _ = self.visit(self.tcx.type_of(self.item_def_id).instantiate_identity());
        self
    }
    fn trait_ref(&mut self) -> &mut Self {
        self.in_primary_interface = true;
        let _ = self.visit_trait(
            self.tcx
                .impl_trait_ref(self.item_def_id)
                .instantiate_identity(),
        );
        self
    }
    fn check_def_id(&self, def_id: DefId, kind: &str, descr: &dyn fmt::Display) -> bool {
        if self.leaks_private_dep(def_id) {
            self.tcx.emit_node_span_lint(
                lint::builtin::EXPORTED_PRIVATE_DEPENDENCIES,
                self.tcx.local_def_id_to_hir_id(self.item_def_id),
                self.tcx.def_span(self.item_def_id.to_def_id()),
                FromPrivateDependencyInPublicInterface {
                    kind,
                    descr: descr.into(),
                    krate: self.tcx.crate_name(def_id.krate),
                },
            );
        }
        let Some(local_def_id) = def_id.as_local() else {
            return false;
        };
        let vis = self.tcx.local_visibility(local_def_id);
        if self.in_assoc_ty && !vis.is_at_least(self.required_visibility, self.tcx) {
            let vis_descr = match vis {
                ty::Visibility::Public => "public",
                ty::Visibility::Restricted(vis_def_id) => {
                    if vis_def_id
                        == self
                            .tcx
                            .parent_module_from_def_id(local_def_id)
                            .to_local_def_id()
                    {
                        "private"
                    } else if vis_def_id.is_top_level_module() {
                        "crate-private"
                    } else {
                        "restricted"
                    }
                }
            };
            let span = self.tcx.def_span(self.item_def_id.to_def_id());
            let vis_span = self.tcx.def_span(def_id);
            self.tcx.dcx().emit_err(InPublicInterface {
                span,
                vis_descr,
                kind,
                descr: descr.into(),
                vis_span,
            });
            return false;
        }
        let Some(effective_vis) = self.required_effective_vis else {
            return false;
        };
        let reachable_at_vis = *effective_vis.at_level(Level::Reachable);
        if !vis.is_at_least(reachable_at_vis, self.tcx) {
            let lint = if self.in_primary_interface {
                lint::builtin::PRIVATE_INTERFACES
            } else {
                lint::builtin::PRIVATE_BOUNDS
            };
            let span = self.tcx.def_span(self.item_def_id.to_def_id());
            let vis_span = self.tcx.def_span(def_id);
            self.tcx.emit_node_span_lint(
                lint,
                self.tcx.local_def_id_to_hir_id(self.item_def_id),
                span,
                PrivateInterfacesOrBoundsLint {
                    item_span: span,
                    item_kind: self.tcx.def_descr(self.item_def_id.to_def_id()),
                    item_descr: (&LazyDefPathStr {
                        def_id: self.item_def_id.to_def_id(),
                        tcx: self.tcx,
                    })
                        .into(),
                    item_vis_descr: &reachable_at_vis.to_string(self.item_def_id, self.tcx),
                    ty_span: vis_span,
                    ty_kind: kind,
                    ty_descr: descr.into(),
                    ty_vis_descr: &vis.to_string(local_def_id, self.tcx),
                },
            );
        }
        false
    }
    /// An item is 'leaked' from a private dependency if all
    /// of the following are true:
    /// 1. It's contained within a public type
    /// 2. It comes from a private crate
    fn leaks_private_dep(&self, item_id: DefId) -> bool {
        let ret = self.required_visibility.is_public() && self.tcx.is_private_dep(item_id.krate);
        debug!("leaks_private_dep(item_id={:?})={}", item_id, ret);
        ret
    }
}
