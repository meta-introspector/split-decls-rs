// Generated macro for macro_140 (macro)
macro_rules! Depcrate_streammacro_140 {
() => {
// Module: crate::stream
// Provides: {"macro_140"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`StreamExt::flat_map()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct FlatMap < S , U , F > { # [pin] stream : Map < S , F >, # [pin] inner_stream : Option < U >, } }
};
}
