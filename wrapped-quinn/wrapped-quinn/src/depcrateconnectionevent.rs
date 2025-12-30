// Generated macro for ConnectionEvent (enum)
macro_rules! DepcrateConnectionEvent {
() => {
// Module: crate
// Provides: {"ConnectionEvent"}
// Dependencies: {}
# [derive (Debug)] enum ConnectionEvent { Close { error_code : VarInt , reason : bytes :: Bytes , } , Proto (proto :: ConnectionEvent) , Rebind (Pin < Box < dyn UdpSender > >) , }
};
}
