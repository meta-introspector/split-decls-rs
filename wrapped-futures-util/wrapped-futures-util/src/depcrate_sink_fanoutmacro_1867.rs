// Generated macro for macro_1867 (macro)
macro_rules! Depcrate_sink_fanoutmacro_1867 {
() => {
// Module: crate::sink::fanout
// Provides: {"macro_1867"}
// Dependencies: {}
pin_project ! { # [doc = " Sink that clones incoming items and forwards them to two sinks at the same time."] # [doc = ""] # [doc = " Backpressure from any downstream sink propagates up, which means that this sink"] # [doc = " can only process items as fast as its _slowest_ downstream sink."] # [must_use = "sinks do nothing unless polled"] pub struct Fanout < Si1 , Si2 > { # [pin] sink1 : Si1 , # [pin] sink2 : Si2 } }
};
}
