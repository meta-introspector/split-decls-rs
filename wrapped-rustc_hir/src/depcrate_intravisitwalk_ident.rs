// Generated macro for walk_ident (function)
macro_rules! Depcrate_intravisitwalk_ident {
() => {
// Module: crate::intravisit
// Provides: {"walk_ident"}
// Dependencies: {}
pub fn walk_ident < 'v , V : Visitor < 'v > > (visitor : & mut V , ident : Ident) -> V :: Result { visitor . visit_name (ident . name) }
};
}
