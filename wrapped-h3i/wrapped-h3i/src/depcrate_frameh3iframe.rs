// Generated macro for H3iFrame (enum)
macro_rules! Depcrate_frameH3iFrame {
() => {
// Module: crate::frame
// Provides: {"H3iFrame"}
// Dependencies: {}
# [doc = " An internal representation of a QUIC or HTTP/3 frame. This type exists so"] # [doc = " that we can extend types defined in Quiche."] # [derive (Debug , Eq , PartialEq , Clone)] pub enum H3iFrame { # [doc = " A wrapper around a quiche HTTP/3 frame."] QuicheH3 (QFrame) , # [doc = " A wrapper around an [EnrichedHeaders] struct."] Headers (EnrichedHeaders) , # [doc = " A wrapper around a [ResetStream] struct"] ResetStream (ResetStream) , }
};
}
