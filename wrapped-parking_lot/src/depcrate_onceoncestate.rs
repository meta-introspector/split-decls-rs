// Generated macro for OnceState (enum)
macro_rules! Depcrate_onceOnceState {
() => {
// Module: crate::once
// Provides: {"OnceState"}
// Dependencies: {}
# [doc = " Current state of a `Once`."] # [derive (Copy , Clone , Eq , PartialEq , Debug)] pub enum OnceState { # [doc = " A closure has not been executed yet"] New , # [doc = " A closure was executed but panicked."] Poisoned , # [doc = " A thread is currently executing a closure."] InProgress , # [doc = " A closure has completed successfully."] Done , }
};
}
