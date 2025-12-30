// Generated macro for macro_161 (macro)
macro_rules! Depcrate_streammacro_161 {
() => {
// Module: crate::stream
// Provides: {"macro_161"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`StreamExt::map_while()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct MapWhile < S , P > { # [pin] stream : S , predicate : P , } }
};
}
