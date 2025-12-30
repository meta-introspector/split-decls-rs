// Generated macro for macro_108 (macro)
macro_rules! Depcrate_streammacro_108 {
() => {
// Module: crate::stream
// Provides: {"macro_108"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`stop_after_future()`] function."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct StopAfterFuture < S : Stream , Fut : Future > { # [pin] stream : S , # [pin] fut : Option < Fut >, fut_result : Option < Fut :: Output >, free : bool , } }
};
}
