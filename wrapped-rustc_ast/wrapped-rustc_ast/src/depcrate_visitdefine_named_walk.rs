// Generated macro for define_named_walk (macro)
macro_rules! Depcrate_visitdefine_named_walk {
() => {
// Module: crate::visit
// Provides: {"define_named_walk"}
// Dependencies: {}
macro_rules ! define_named_walk { ($ Visitor : ident <$ lt : lifetime > $ (pub fn $ method : ident ($ ty : ty) ;) *) => { $ (pub fn $ method <$ lt , V : $ Visitor <$ lt >> (visitor : & mut V , node : &$ lt $ ty) -> V :: Result { walk_walkable ! (visitor , node ,) }) * } ; }
};
}
