// Generated macro for macro_1576 (macro)
macro_rules! Depcrate_stream_oncemacro_1576 {
() => {
// Module: crate::stream::once
// Provides: {"macro_1576"}
// Dependencies: {}
pin_project ! { # [doc = " A stream which emits single element and then EOF."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Once < Fut > { # [pin] future : Option < Fut > } }
};
}
