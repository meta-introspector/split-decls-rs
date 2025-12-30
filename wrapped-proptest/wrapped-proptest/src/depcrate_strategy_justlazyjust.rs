// Generated macro for LazyJust (struct)
macro_rules! Depcrate_strategy_justLazyJust {
() => {
// Module: crate::strategy::just
// Provides: {"LazyJust"}
// Dependencies: {}
# [doc = " A `Strategy` which always produces a single value value and never"] # [doc = " simplifies. If `T` is `Clone`, you should use `Just` instead."] # [doc = ""] # [doc = " This is a generalization of `Just` and works by calling"] # [doc = " the provided `Fn () -> T` in `.current()` every time. This is not a"] # [doc = " very interesting strategy, but is required in cases where `T` is"] # [doc = " not `Clone`. It is also used in `proptest_derive` where we can't"] # [doc = " assume that your type is `Clone`."] # [doc = ""] # [doc = " **It is important that the function used be pure.**"] # [must_use = "strategies do nothing unless used"] pub struct LazyJust < T , F : Fn () -> T > { # [doc = " The function executed in `.current()`."] function : F , }
};
}
