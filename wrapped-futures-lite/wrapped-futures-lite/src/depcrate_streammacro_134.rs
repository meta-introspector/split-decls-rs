// Generated macro for macro_134 (macro)
macro_rules! Depcrate_streammacro_134 {
() => {
// Module: crate::stream
// Provides: {"macro_134"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`StreamExt::scan()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Scan < S , St , F > { # [pin] stream : S , state_f : (St , F) , } }
};
}
