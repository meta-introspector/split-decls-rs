// Generated macro for ReservedTransportParameter (struct)
macro_rules! Depcrate_transport_parametersReservedTransportParameter {
() => {
// Module: crate::transport_parameters
// Provides: {"ReservedTransportParameter"}
// Dependencies: {}
# [doc = " A reserved transport parameter."] # [doc = ""] # [doc = " It has an identifier of the form 31 * N + 27 for the integer value of N."] # [doc = " Such identifiers are reserved to exercise the requirement that unknown transport parameters be ignored."] # [doc = " The reserved transport parameter has no semantics and can carry arbitrary values."] # [doc = " It may be included in transport parameters sent to the peer, and should be ignored when received."] # [doc = ""] # [doc = " See spec: <https://www.rfc-editor.org/rfc/rfc9000.html#section-18.1>"] # [derive (Debug , Copy , Clone , Eq , PartialEq)] pub (crate) struct ReservedTransportParameter { # [doc = " The reserved identifier of the transport parameter"] id : VarInt , # [doc = " Buffer to store the parameter payload"] payload : [u8 ; Self :: MAX_PAYLOAD_LEN] , # [doc = " The number of bytes to include in the wire format from the `payload` buffer"] payload_len : usize , }
};
}
