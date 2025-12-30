// Generated macro for macro_1899 (macro)
macro_rules! Depcrate_sink_err_intomacro_1899 {
() => {
// Module: crate::sink::err_into
// Provides: {"macro_1899"}
// Dependencies: {}
pin_project ! { # [doc = " Sink for the [`sink_err_into`](super::SinkExt::sink_err_into) method."] # [derive (Debug)] # [must_use = "sinks do nothing unless polled"] pub struct SinkErrInto < Si : Sink < Item >, Item , E > { # [pin] sink : SinkMapErr < Si , fn (Si :: Error) -> E >, } }
};
}
