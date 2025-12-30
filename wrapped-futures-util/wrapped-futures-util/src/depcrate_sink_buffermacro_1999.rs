// Generated macro for macro_1999 (macro)
macro_rules! Depcrate_sink_buffermacro_1999 {
() => {
// Module: crate::sink::buffer
// Provides: {"macro_1999"}
// Dependencies: {}
pin_project ! { # [doc = " Sink for the [`buffer`](super::SinkExt::buffer) method."] # [derive (Debug)] # [must_use = "sinks do nothing unless polled"] pub struct Buffer < Si , Item > { # [pin] sink : Si , buf : VecDeque < Item >, capacity : usize , } }
};
}
