// Generated macro for impl_walkable (macro)
macro_rules! Depcrate_mut_visitimpl_walkable {
() => {
// Module: crate::mut_visit
// Provides: {"impl_walkable"}
// Dependencies: {}
macro_rules ! impl_walkable { ($ (<$ K : ident : $ Kb : ident >) ? |& mut $ self : ident : $ self_ty : ty , $ vis : ident : & mut $ vis_ty : ident | $ block : block) => { # [allow (unused_parens , non_local_definitions)] impl <$ ($ K : $ Kb ,) ? $ vis_ty : MutVisitor > MutWalkable <$ vis_ty > for $ self_ty { fn walk_mut (& mut $ self , $ vis : & mut $ vis_ty) -> V :: Result { $ block } } } ; }
};
}
