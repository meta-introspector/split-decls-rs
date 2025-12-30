// Generated macro for macro_1237 (macro)
macro_rules! Depcrate_sslmacro_1237 {
() => {
// Module: crate::ssl
// Provides: {"macro_1237"}
// Dependencies: {}
bitflags ! { # [doc = " The shutdown state of a session."] # [derive (Copy , Clone , Debug , Eq , Hash , Ord , PartialEq , PartialOrd)] # [repr (transparent)] pub struct ShutdownState : c_int { # [doc = " A close notify message has been sent to the peer."] const SENT = ffi :: SSL_SENT_SHUTDOWN ; # [doc = " A close notify message has been received from the peer."] const RECEIVED = ffi :: SSL_RECEIVED_SHUTDOWN ; } }
};
}
