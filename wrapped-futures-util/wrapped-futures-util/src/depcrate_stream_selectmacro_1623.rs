// Generated macro for macro_1623 (macro)
macro_rules! Depcrate_stream_selectmacro_1623 {
() => {
// Module: crate::stream::select
// Provides: {"macro_1623"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`select()`] function."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Select < St1 , St2 > { # [pin] inner : SelectWithStrategy < St1 , St2 , fn (& mut PollNext) -> PollNext , PollNext >, } }
};
}
