// Generated macro for generate_walk_flat_map_fns (macro)
macro_rules! Depcrate_mut_visitgenerate_walk_flat_map_fns {
() => {
// Module: crate::mut_visit
// Provides: {"generate_walk_flat_map_fns"}
// Dependencies: {}
macro_rules ! generate_walk_flat_map_fns { ($ ($ fn_name : ident ($ Ty : ty $ (,$ extra_name : ident : $ ExtraTy : ty) *) => $ visit_fn_name : ident ;) +) => { $ (pub fn $ fn_name < V : MutVisitor > (vis : & mut V , mut value : $ Ty $ (,$ extra_name : $ ExtraTy) *) -> SmallVec < [$ Ty ; 1] > { vis .$ visit_fn_name (& mut value $ (,$ extra_name) *) ; smallvec ! [value] }) + } ; }
};
}
