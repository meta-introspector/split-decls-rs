// Generated macro for parse_depth (function)
macro_rules! Depcrate_parserparse_depth {
() => {
// Module: crate::parser
// Provides: {"parse_depth"}
// Dependencies: {}
fn parse_depth (src : & mut TtIter < '_ , Span >) -> Result < usize , () > { if src . is_empty () { Ok (0) } else if let tt :: Leaf :: Literal (tt :: Literal { symbol : text , suffix : None , .. }) = src . expect_literal () ? { text . as_str () . parse () . map_err (| _ | ()) } else { Err (()) } }
};
}
