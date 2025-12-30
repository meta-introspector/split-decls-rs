// Generated macro for macro_179 (macro)
macro_rules! Depcrate_streammacro_179 {
() => {
// Module: crate::stream
// Provides: {"macro_179"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`StreamExt::inspect()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Inspect < S , F > { # [pin] stream : S , f : F , } }
};
}
