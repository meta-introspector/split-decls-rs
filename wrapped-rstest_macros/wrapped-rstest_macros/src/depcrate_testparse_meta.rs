// Generated macro for parse_meta (function)
macro_rules! Depcrate_testparse_meta {
() => {
// Module: crate::test
// Provides: {"parse_meta"}
// Dependencies: {}
pub (crate) fn parse_meta < T : syn :: parse :: Parse , S : AsRef < str > > (test_case : S) -> T { let to_parse = format ! (r#"
        #[outer({})]
        fn to_parse() {{}}
        "# , test_case . as_ref ()) ; let item_fn = parse_str :: < ItemFn > (& to_parse) . expect (& format ! ("Cannot parse '{}'" , to_parse)) ; let tokens = quote ! (# item_fn) ; let tt = tokens . into_iter () . skip (1) . next () . unwrap () ; if let TokenTree :: Group (g) = tt { let ts = g . stream () ; parse2 :: < Outer < T > > (ts) . unwrap () . 0 } else { panic ! ("Cannot find group in {:#?}" , tt) } }
};
}
