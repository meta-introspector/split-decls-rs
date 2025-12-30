// Generated macro for macro_142 (macro)
macro_rules! Depcrate_streammacro_142 {
() => {
// Module: crate::stream
// Provides: {"macro_142"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`StreamExt::flatten()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Flatten < S : Stream > { # [pin] stream : S , # [pin] inner_stream : Option < S :: Item >, } }
};
}
