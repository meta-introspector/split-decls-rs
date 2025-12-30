// Generated macro for Action (enum)
macro_rules! Depcrate_walkAction {
() => {
// Module: crate::walk
// Provides: {"Action"}
// Dependencies: {}
# [doc = " A type returned by the [`Delegate::emit()`] as passed to [`walk()`](function::walk())."] # [derive (Debug , Copy , Clone , Eq , PartialEq)] # [must_use] pub enum Action { # [doc = " Continue the traversal as normal."] Continue , # [doc = " Do not continue the traversal, but exit it."] Cancel , }
};
}
