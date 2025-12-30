// Generated macro for Can (trait)
macro_rules! Depcrate_blockingCan {
() => {
// Module: crate::blocking
// Provides: {"Can"}
// Dependencies: {}
# [doc = " A blocking CAN interface that is able to transmit and receive frames."] pub trait Can { # [doc = " Associated frame type."] type Frame : crate :: Frame ; # [doc = " Associated error type."] type Error : crate :: Error ; # [doc = " Puts a frame in the transmit buffer. Blocks until space is available in"] # [doc = " the transmit buffer."] fn transmit (& mut self , frame : & Self :: Frame) -> Result < () , Self :: Error > ; # [doc = " Blocks until a frame was received or an error occurred."] fn receive (& mut self) -> Result < Self :: Frame , Self :: Error > ; }
};
}
