// Generated macro for of (function)
macro_rules! Depcrate_optionof {
() => {
// Module: crate::option
// Provides: {"of"}
// Dependencies: {}
# [doc = " Return a strategy producing `Optional` values wrapping values from the"] # [doc = " given delegate strategy."] # [doc = ""] # [doc = " `Some` values shrink to `None`."] # [doc = ""] # [doc = " `Some` and `None` are each chosen with 50% probability."] pub fn of < T : Strategy > (t : T) -> OptionStrategy < T > { weighted (Probability :: default () , t) }
};
}
