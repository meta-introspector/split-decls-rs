// Generated macro for DatagramConnectionEvent (struct)
macro_rules! Depcrate_sharedDatagramConnectionEvent {
() => {
// Module: crate::shared
// Provides: {"DatagramConnectionEvent"}
// Dependencies: {}
# [doc = " Variant of [`ConnectionEventInner`]."] # [derive (Debug)] pub (crate) struct DatagramConnectionEvent { pub (crate) now : Instant , pub (crate) remote : SocketAddr , pub (crate) ecn : Option < EcnCodepoint > , pub (crate) first_decode : PartialDecode , pub (crate) remaining : Option < BytesMut > , }
};
}
