// Generated macro for maybe_err (function)
macro_rules! Depcrate_resultmaybe_err {
() => {
// Module: crate::result
// Provides: {"maybe_err"}
// Dependencies: {}
# [doc = " Create a strategy for `Result`s where `Ok` values are taken from `t` and"] # [doc = " `Err` values are taken from `e`."] # [doc = ""] # [doc = " `Ok` and `Err` are chosen with equal probability."] # [doc = ""] # [doc = " Generated values shrink to `Ok`."] pub fn maybe_err < T : Strategy , E : Strategy > (t : T , e : E) -> MaybeErr < T , E > { maybe_err_weighted (0.5 , t , e) }
};
}
