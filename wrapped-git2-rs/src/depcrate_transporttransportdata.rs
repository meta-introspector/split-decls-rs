// Generated macro for TransportData (struct)
macro_rules! Depcrate_transportTransportData {
() => {
// Module: crate::transport
// Provides: {"TransportData"}
// Dependencies: {}
# [doc = " Boxed data payload used for registering new transports."] # [doc = ""] # [doc = " Currently only contains a field which knows how to create transports."] struct TransportData { factory : Box < TransportFactory > , }
};
}
