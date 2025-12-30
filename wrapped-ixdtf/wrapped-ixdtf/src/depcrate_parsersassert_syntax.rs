// Generated macro for assert_syntax (macro)
macro_rules! Depcrate_parsersassert_syntax {
() => {
// Module: crate::parsers
// Provides: {"assert_syntax"}
// Dependencies: {}
# [doc = " `assert_syntax!` is a parser specific utility macro for asserting a syntax test, and returning the"] # [doc = " the provided provided error if the assertion fails."] # [macro_export] macro_rules ! assert_syntax { ($ cond : expr , $ err : ident $ (,) ?) => { if !$ cond { return Err (ParseError ::$ err) ; } } ; }
};
}
