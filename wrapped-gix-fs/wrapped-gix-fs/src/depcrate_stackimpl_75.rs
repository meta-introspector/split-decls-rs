// Generated macro for impl_75 (impl)
macro_rules! Depcrate_stackimpl_75 {
() => {
// Module: crate::stack
// Provides: {"impl_75"}
// Dependencies: {}
# [doc = " Access"] impl Stack { # [doc = " Returns the top-level path of the stack."] pub fn root (& self) -> & Path { & self . root } # [doc = " Returns the absolute path the currently set path."] pub fn current (& self) -> & Path { & self . current } # [doc = " Returns the currently set path relative to the [`root()`][Stack::root()]."] pub fn current_relative (& self) -> & Path { & self . current_relative } }
};
}
