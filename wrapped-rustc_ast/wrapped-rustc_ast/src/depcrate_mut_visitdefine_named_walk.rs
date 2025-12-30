// Generated macro for define_named_walk (macro)
macro_rules! Depcrate_mut_visitdefine_named_walk {
() => {
// Module: crate::mut_visit
// Provides: {"define_named_walk"}
// Dependencies: {}
macro_rules ! define_named_walk { ((mut) $ Visitor : ident $ (pub fn $ method : ident ($ ty : ty) ;) *) => { $ (pub fn $ method < V : $ Visitor > (visitor : & mut V , node : & mut $ ty) { walk_walkable ! (visitor , node , mut) }) * } ; }
};
}
