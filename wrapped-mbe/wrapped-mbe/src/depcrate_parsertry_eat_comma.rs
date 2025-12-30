// Generated macro for try_eat_comma (function)
macro_rules! Depcrate_parsertry_eat_comma {
() => {
// Module: crate::parser
// Provides: {"try_eat_comma"}
// Dependencies: {}
fn try_eat_comma (src : & mut TtIter < '_ , Span >) -> bool { if let Some (TtElement :: Leaf (tt :: Leaf :: Punct (tt :: Punct { char : ',' , .. }))) = src . peek () { let _ = src . next () ; return true ; } false }
};
}
