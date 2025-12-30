// Generated macro for Recv (struct)
macro_rules! Depcrate_mpscRecv {
() => {
// Module: crate::mpsc
// Provides: {"Recv"}
// Dependencies: {}
# [doc = " Future returned by [`Receiver::recv()`] or [`UnboundedReceiver::recv()`]."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Recv < 'a , St : ? Sized > { stream : & 'a mut St , }
};
}
