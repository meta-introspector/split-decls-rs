// Generated macro for impl_858 (impl)
macro_rules! Depcrate_strategy_justimpl_858 {
() => {
// Module: crate::strategy::just
// Provides: {"impl_858"}
// Dependencies: {}
impl < T , F : Fn () -> T > LazyJust < T , F > { # [doc = " Constructs a `LazyJust` strategy given the function/closure"] # [doc = " that produces the value."] # [doc = ""] # [doc = " **It is important that the function used be pure.**"] pub fn new (function : F) -> Self { Self { function } } }
};
}
