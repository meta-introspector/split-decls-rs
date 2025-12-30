// Generated macro for macro_155 (macro)
macro_rules! Depcrate_streammacro_155 {
() => {
// Module: crate::stream
// Provides: {"macro_155"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`StreamExt::filter_map()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct FilterMap < S , F > { # [pin] stream : S , f : F , } }
};
}
