// Generated macro for macro_1911 (macro)
macro_rules! Depcrate_sink_map_errmacro_1911 {
() => {
// Module: crate::sink::map_err
// Provides: {"macro_1911"}
// Dependencies: {}
pin_project ! { # [doc = " Sink for the [`sink_map_err`](super::SinkExt::sink_map_err) method."] # [derive (Debug , Clone)] # [must_use = "sinks do nothing unless polled"] pub struct SinkMapErr < Si , F > { # [pin] sink : Si , f : Option < F >, } }
};
}
