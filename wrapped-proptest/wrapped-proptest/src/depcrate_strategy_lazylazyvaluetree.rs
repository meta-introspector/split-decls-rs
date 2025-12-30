// Generated macro for LazyValueTree (struct)
macro_rules! Depcrate_strategy_lazyLazyValueTree {
() => {
// Module: crate::strategy::lazy
// Provides: {"LazyValueTree"}
// Dependencies: {}
# [doc = " Represents a value tree that is initialized on the first call to any"] # [doc = " methods."] # [doc = ""] # [doc = " This is used to defer potentially expensive generation to shrinking time. It"] # [doc = " is public only to allow APIs to expose it as an intermediate value."] pub struct LazyValueTree < S : Strategy > { state : LazyValueTreeState < S > , }
};
}
