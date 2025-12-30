// Generated macro for IntoInnerError (struct)
macro_rules! Depcrate_errorIntoInnerError {
() => {
// Module: crate::error
// Provides: {"IntoInnerError"}
// Dependencies: {}
# [doc = " `IntoInnerError` occurs when consuming a `Writer` fails."] # [doc = ""] # [doc = " Consuming the `Writer` causes a flush to happen. If the flush fails, then"] # [doc = " this error is returned, which contains both the original `Writer` and"] # [doc = " the error that occurred."] # [doc = ""] # [doc = " The type parameter `W` is the unconsumed writer."] pub struct IntoInnerError < W > { wtr : W , err : io :: Error , }
};
}
