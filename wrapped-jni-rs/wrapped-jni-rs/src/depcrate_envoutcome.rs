// Generated macro for Outcome (enum)
macro_rules! Depcrate_envOutcome {
() => {
// Module: crate::env
// Provides: {"Outcome"}
// Dependencies: {}
# [doc = " The outcome of an operation that may succeed, fail with an error, or panic."] # [doc = ""] # [doc = " This enum is used to encapsulate the result of operations within native methods"] # [doc = " where extra care is needed to handle errors and panics gracefully before"] # [doc = " returning a value to the Java environment."] # [derive (Debug)] pub enum Outcome < T , E > { # [doc = " Contains the success value"] Ok (T) , # [doc = " Contains the error value"] Err (E) , # [doc = " Contains the panic value"] Panic (Box < dyn std :: any :: Any + Send + 'static >) , }
};
}
