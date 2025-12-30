// Generated macro for Complete (struct)
macro_rules! Depcrate_promiseComplete {
() => {
// Module: crate::promise
// Provides: {"Complete"}
// Dependencies: {}
# [doc = " Represents the completion half of a promise through which the result of a"] # [doc = " computation is signaled."] # [doc = ""] # [doc = " This is created by the `promise` function."] pub struct Complete < T > where T : Send + 'static , { inner : Arc < Inner < T > > , completed : bool , }
};
}
