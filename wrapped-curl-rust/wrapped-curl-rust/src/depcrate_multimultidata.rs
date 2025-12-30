// Generated macro for MultiData (struct)
macro_rules! Depcrate_multiMultiData {
() => {
// Module: crate::multi
// Provides: {"MultiData"}
// Dependencies: {}
struct MultiData { socket : Box < dyn FnMut (Socket , SocketEvents , usize) + Send > , timer : Box < dyn FnMut (Option < Duration >) -> bool + Send > , }
};
}
