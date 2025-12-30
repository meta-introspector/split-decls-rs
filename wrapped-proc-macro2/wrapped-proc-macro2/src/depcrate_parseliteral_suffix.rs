// Generated macro for literal_suffix (function)
macro_rules! Depcrate_parseliteral_suffix {
() => {
// Module: crate::parse
// Provides: {"literal_suffix"}
// Dependencies: {}
fn literal_suffix (input : Cursor) -> Cursor { match ident_not_raw (input) { Ok ((input , _)) => input , Err (Reject) => input , } }
};
}
