// Generated macro for leaf_token (function)
macro_rules! Depcrate_parseleaf_token {
() => {
// Module: crate::parse
// Provides: {"leaf_token"}
// Dependencies: {}
fn leaf_token (input : Cursor) -> PResult < TokenTree > { if let Ok ((input , l)) = literal (input) { Ok ((input , TokenTree :: Literal (crate :: Literal :: _new_fallback (l)))) } else if let Ok ((input , p)) = punct (input) { Ok ((input , TokenTree :: Punct (p))) } else if let Ok ((input , i)) = ident (input) { Ok ((input , TokenTree :: Ident (i))) } else if input . starts_with (ERROR) { let rest = input . advance (ERROR . len ()) ; let repr = crate :: Literal :: _new_fallback (Literal :: _new (ERROR . to_owned ())) ; Ok ((rest , TokenTree :: Literal (repr))) } else { Err (Reject) } }
};
}
