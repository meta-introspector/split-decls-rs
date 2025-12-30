// Generated macro for macro_180 (macro)
macro_rules! Depcrate_channelmacro_180 {
() => {
// Module: crate::channel
// Provides: {"macro_180"}
// Dependencies: {}
pin_project ! { # [doc = " A body backed by a channel."] pub struct Channel < D , E = std :: convert :: Infallible > { rx_frame : mpsc :: Receiver < Frame < D >>, # [pin] rx_error : oneshot :: Receiver < E >, } }
};
}
