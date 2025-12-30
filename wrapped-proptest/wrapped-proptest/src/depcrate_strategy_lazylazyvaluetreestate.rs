// Generated macro for LazyValueTreeState (enum)
macro_rules! Depcrate_strategy_lazyLazyValueTreeState {
() => {
// Module: crate::strategy::lazy
// Provides: {"LazyValueTreeState"}
// Dependencies: {}
enum LazyValueTreeState < S : Strategy > { Initialized (S :: Tree) , Uninitialized { strategy : Arc < S > , runner : TestRunner , } , Failed , }
};
}
