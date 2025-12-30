// Generated macro for Deallocate (enum)
macro_rules! Depcrate_abiDeallocate {
() => {
// Module: crate::abi
// Provides: {"Deallocate"}
// Dependencies: {}
# [doc = " What to deallocate in various `deallocate_*` methods."] # [derive (Copy , Clone)] enum Deallocate { # [doc = " Only deallocate lists."] Lists , # [doc = " Deallocate lists and owned resources such as `own<T>` and"] # [doc = " futures/streams."] ListsAndOwn , }
};
}
