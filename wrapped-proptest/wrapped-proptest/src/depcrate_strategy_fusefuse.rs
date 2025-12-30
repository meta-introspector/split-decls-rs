// Generated macro for Fuse (struct)
macro_rules! Depcrate_strategy_fuseFuse {
() => {
// Module: crate::strategy::fuse
// Provides: {"Fuse"}
// Dependencies: {}
# [doc = " Adaptor for `Strategy` and `ValueTree` which guards `simplify()` and"] # [doc = " `complicate()` to avoid contract violations."] # [doc = ""] # [doc = " This can be used as an intermediate when the caller would otherwise need"] # [doc = " its own separate state tracking, or as a workaround for a broken"] # [doc = " `ValueTree` implementation."] # [doc = ""] # [doc = " This wrapper specifically has the following effects:"] # [doc = ""] # [doc = " - Calling `complicate()` before `simplify()` was ever called does nothing"] # [doc = "   and returns `false`."] # [doc = ""] # [doc = " - Calling `simplify()` after it has returned `false` and no calls to"] # [doc = "   `complicate()` returned `true` does nothing and returns `false`."] # [doc = ""] # [doc = " - Calling `complicate()` after it has returned `false` and no calls to"] # [doc = "   `simplify()` returned `true` does nothing and returns `false`."] # [doc = ""] # [doc = " There is also limited functionality to alter the internal state to assist"] # [doc = " in its usage as a state tracker."] # [doc = ""] # [doc = " Wrapping a `Strategy` in `Fuse` simply causes its `ValueTree` to also be"] # [doc = " wrapped in `Fuse`."] # [doc = ""] # [doc = " While this is similar to `std::iter::Fuse`, it is not exposed as a method"] # [doc = " on `Strategy` since the vast majority of proptest should never need this"] # [doc = " functionality; it mainly concerns implementors of strategies."] # [derive (Debug , Clone , Copy)] # [must_use = "strategies do nothing unless used"] pub struct Fuse < T > { inner : T , may_simplify : bool , may_complicate : bool , }
};
}
