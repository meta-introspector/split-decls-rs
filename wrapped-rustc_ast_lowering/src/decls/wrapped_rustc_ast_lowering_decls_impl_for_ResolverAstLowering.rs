use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[extension(trait ResolverAstLoweringExt)]
impl ResolverAstLowering {
    fn legacy_const_generic_args(&self, expr: &Expr) -> Option<Vec<usize>> {
        if let ExprKind::Path(None, path) = &expr.kind {
            if path.segments.last().unwrap().args.is_some() {
                return None;
            }
            if let Res::Def(DefKind::Fn, def_id) = self
                .partial_res_map
                .get(&expr.id)?
                .full_res()?
            {
                if def_id.is_local() {
                    return None;
                }
                if let Some(v) = self.legacy_const_generic_args.get(&def_id) {
                    return v.clone();
                }
            }
        }
        None
    }
    fn get_partial_res(&self, id: NodeId) -> Option<PartialRes> {
        self.partial_res_map.get(&id).copied()
    }
    /// Obtains per-namespace resolutions for `use` statement with the given `NodeId`.
    fn get_import_res(&self, id: NodeId) -> PerNS<Option<Res<NodeId>>> {
        self.import_res_map.get(&id).copied().unwrap_or_default()
    }
    /// Obtains resolution for a label with the given `NodeId`.
    fn get_label_res(&self, id: NodeId) -> Option<NodeId> {
        self.label_res_map.get(&id).copied()
    }
    /// Obtains resolution for a lifetime with the given `NodeId`.
    fn get_lifetime_res(&self, id: NodeId) -> Option<LifetimeRes> {
        self.lifetimes_res_map.get(&id).copied()
    }
    /// Obtain the list of lifetimes parameters to add to an item.
    ///
    /// Extra lifetime parameters should only be added in places that can appear
    /// as a `binder` in `LifetimeRes`.
    ///
    /// The extra lifetimes that appear from the parenthesized `Fn`-trait desugaring
    /// should appear at the enclosing `PolyTraitRef`.
    fn extra_lifetime_params(
        &mut self,
        id: NodeId,
    ) -> Vec<(Ident, NodeId, LifetimeRes)> {
        self.extra_lifetime_params_map.get(&id).cloned().unwrap_or_default()
    }
}
