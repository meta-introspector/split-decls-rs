// Generated macro for Promise (struct)
macro_rules! Depcrate_promisePromise {
() => {
// Module: crate::promise
// Provides: {"Promise"}
// Dependencies: {}
# [doc = " A future representing the completion of a computation happening elsewhere in"] # [doc = " memory."] # [doc = ""] # [doc = " This is created by the `promise` function."] pub struct Promise < T > where T : Send + 'static , { inner : Arc < Inner < T > > , cancel_token : Option < Token > , }
};
}
