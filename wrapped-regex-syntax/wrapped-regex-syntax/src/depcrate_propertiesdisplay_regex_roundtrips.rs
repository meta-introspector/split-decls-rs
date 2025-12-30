// Generated macro for display_regex_roundtrips (function)
macro_rules! Depcrate_propertiesdisplay_regex_roundtrips {
() => {
// Module: crate::properties
// Provides: {"display_regex_roundtrips"}
// Dependencies: {}
# [test] fn display_regex_roundtrips () { fn prop (e : Expr) -> bool { let parser = ExprBuilder :: new () . allow_bytes (true) ; e == parser . parse (& e . to_string ()) . unwrap () } QuickCheck :: new () . tests (10_000) . max_tests (20_000) . gen (StdGen :: new (:: rand :: thread_rng () , 50)) . quickcheck (prop as fn (Expr) -> bool) ; }
};
}
