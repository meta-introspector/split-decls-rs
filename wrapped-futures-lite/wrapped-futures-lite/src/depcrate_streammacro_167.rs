// Generated macro for macro_167 (macro)
macro_rules! Depcrate_streammacro_167 {
() => {
// Module: crate::stream
// Provides: {"macro_167"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`StreamExt::step_by()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct StepBy < S > { # [pin] stream : S , step : usize , i : usize , } }
};
}
