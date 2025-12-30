// Generated macro for PORT_TOTAL_MAXIMUM_MESSAGE_LENGTH (const)
macro_rules! Depcrate_ntlpcapiPORT_TOTAL_MAXIMUM_MESSAGE_LENGTH {
() => {
// Module: crate::ntlpcapi
// Provides: {"PORT_TOTAL_MAXIMUM_MESSAGE_LENGTH"}
// Dependencies: {}
pub const PORT_TOTAL_MAXIMUM_MESSAGE_LENGTH : u32 = (PORT_MAXIMUM_MESSAGE_LENGTH + size_of :: < PORT_MESSAGE > () as u32 + LPC_MAX_CONNECTION_INFO_SIZE + 0xf) & ! 0xf ;
};
}
