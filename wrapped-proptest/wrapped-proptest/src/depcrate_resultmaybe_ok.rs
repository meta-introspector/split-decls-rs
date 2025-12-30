// Generated macro for maybe_ok (function)
macro_rules! Depcrate_resultmaybe_ok {
() => {
// Module: crate::result
// Provides: {"maybe_ok"}
// Dependencies: {}
# [doc = " Create a strategy for `Result`s where `Ok` values are taken from `t` and"] # [doc = " `Err` values are taken from `e`."] # [doc = ""] # [doc = " `Ok` and `Err` are chosen with equal probability."] # [doc = ""] # [doc = " Generated values shrink to `Err`."] pub fn maybe_ok < T : Strategy , E : Strategy > (t : T , e : E) -> MaybeOk < T , E > { maybe_ok_weighted (0.5 , t , e) }
};
}
