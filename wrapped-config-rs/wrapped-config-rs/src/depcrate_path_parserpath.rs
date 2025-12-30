// Generated macro for path (function)
macro_rules! Depcrate_path_parserpath {
() => {
// Module: crate::path::parser
// Provides: {"path"}
// Dependencies: {}
fn path (i : & mut & str) -> ModalResult < Expression > { let root = ident . parse_next (i) ? ; let postfix = repeat (0 .. , postfix) . parse_next (i) ? ; let expr = Expression { root , postfix } ; Ok (expr) }
};
}
