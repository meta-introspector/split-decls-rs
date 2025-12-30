// Generated macro for tests (module)
macro_rules! Depcrate_property_test_utilstests {
() => {
// Module: crate::property_test::utils
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use quote :: ToTokens ; use syn :: parse_quote ; use super :: * ; # [test] fn strip_args_works () { let f = parse_quote ! { fn foo (i : i32) { } } ; let (f , mut args) = strip_args (f) ; assert_eq ! (f . to_token_stream () . to_string () , "fn foo () { }") ; assert_eq ! (args . len () , 1) ; let arg = args . pop () . unwrap () ; assert_eq ! (arg . pat_ty . to_token_stream () . to_string () , "i : i32") ; assert ! (arg . strategy . is_none ()) ; } # [test] # [should_panic] fn strip_args_panics_with_self () { let f = parse_quote ! { fn foo (self) { } } ; strip_args (f) ; } # [test] fn is_strategy_works () { let attr = parse_quote ! { # [strategy = 123] } ; assert ! (is_strategy (& attr)) ; let attr = parse_quote ! { #! [strategy = 123] } ; assert ! (! is_strategy (& attr)) ; let attr = parse_quote ! { # [not_strategy = 123] } ; assert ! (! is_strategy (& attr)) ; let attr = parse_quote ! { # [strategy (but , no , equals)] } ; assert ! (! is_strategy (& attr)) ; let attr = parse_quote ! { # [strategy] } ; assert ! (! is_strategy (& attr)) ; } # [test] fn strip_strategy_works () { let f = parse_quote ! { fn foo (# [strategy = 123] x : i32) { } } ; let Argument { pat_ty , strategy } = strip_args (f) . 1 . pop () . unwrap () ; assert_eq ! (pat_ty . to_token_stream () . to_string () , "x : i32") ; assert_eq ! (strategy . to_token_stream () . to_string () , "123") ; } }
};
}
