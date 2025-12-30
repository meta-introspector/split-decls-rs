// Generated macro for State (enum)
macro_rules! Depcrate_once_cellState {
() => {
// Module: crate::once_cell
// Provides: {"State"}
// Dependencies: {}
# [doc = " The current state of the `OnceCell`."] # [derive (Copy , Clone , PartialEq , Eq)] # [repr (usize)] enum State { # [doc = " The `OnceCell` is uninitialized."] Uninitialized = 0 , # [doc = " The `OnceCell` is being initialized."] Initializing = 1 , # [doc = " The `OnceCell` is initialized."] Initialized = 2 , }
};
}
