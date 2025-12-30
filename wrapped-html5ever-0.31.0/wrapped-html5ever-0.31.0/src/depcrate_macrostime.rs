// Generated macro for time (macro)
macro_rules! Depcrate_macrostime {
() => {
// Module: crate::macros
// Provides: {"time"}
// Dependencies: {}
macro_rules ! time { ($ e : expr) => { { let now = :: std :: time :: Instant :: now () ; let result = $ e ; let d = now . elapsed () ; let dt = d . as_secs () * 1_000_000_000 + u64 :: from (d . subsec_nanos ()) ; (result , dt) } } ; }
};
}
