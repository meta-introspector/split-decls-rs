// Generated macro for Can (trait)
macro_rules! Depcrate_nbCan {
() => {
// Module: crate::nb
// Provides: {"Can"}
// Dependencies: {}
# [doc = " A CAN interface that is able to transmit and receive frames."] pub trait Can { # [doc = " Associated frame type."] type Frame : crate :: Frame ; # [doc = " Associated error type."] type Error : crate :: Error ; # [doc = " Puts a frame in the transmit buffer to be sent on the bus."] # [doc = ""] # [doc = " If the transmit buffer is full, this function will try to replace a pending"] # [doc = " lower priority frame and return the frame that was replaced."] # [doc = " Returns `Err(WouldBlock)` if the transmit buffer is full and no frame can be"] # [doc = " replaced."] # [doc = ""] # [doc = " # Notes for implementers"] # [doc = ""] # [doc = " * Frames of equal identifier shall be transmitted in FIFO fashion when more"] # [doc = "   than one transmit buffer is available."] # [doc = " * When replacing pending frames make sure the frame is not in the process of"] # [doc = "   being send to the bus."] fn transmit (& mut self , frame : & Self :: Frame) -> nb :: Result < Option < Self :: Frame > , Self :: Error > ; # [doc = " Returns a received frame if available."] fn receive (& mut self) -> nb :: Result < Self :: Frame , Self :: Error > ; }
};
}
