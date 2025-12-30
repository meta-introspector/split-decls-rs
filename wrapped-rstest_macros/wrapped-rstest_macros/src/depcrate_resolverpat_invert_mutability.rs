// Generated macro for pat_invert_mutability (function)
macro_rules! Depcrate_resolverpat_invert_mutability {
() => {
// Module: crate::resolver
// Provides: {"pat_invert_mutability"}
// Dependencies: {}
pub (crate) fn pat_invert_mutability (p : & Pat) -> Pat { match p . clone () { Pat :: Ident (mut ident) => { ident . mutability = match ident . mutability { Some (_) => None , None => Some (syn :: parse_quote ! { mut }) , } ; syn :: Pat :: Ident (ident) } p => p , } }
};
}
