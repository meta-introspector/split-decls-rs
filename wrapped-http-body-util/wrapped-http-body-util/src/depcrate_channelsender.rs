// Generated macro for Sender (struct)
macro_rules! Depcrate_channelSender {
() => {
// Module: crate::channel
// Provides: {"Sender"}
// Dependencies: {}
# [doc = " A sender half created through [`Channel::new`]."] pub struct Sender < D , E = std :: convert :: Infallible > { tx_frame : mpsc :: Sender < Frame < D > > , tx_error : oneshot :: Sender < E > , }
};
}
