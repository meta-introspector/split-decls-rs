// Generated macro for impl_189 (impl)
macro_rules! Depcrate_client_legacy_connect_proxy_socks_v5_messagesimpl_189 {
() => {
// Module: crate::client::legacy::connect::proxy::socks::v5::messages
// Provides: {"impl_189"}
// Dependencies: {}
impl NegotiationReq < '_ > { pub fn write_to_buf (& self , buf : & mut BytesMut) -> Result < usize , SerializeError > { if buf . capacity () - buf . len () < 3 { return Err (SerializeError :: WouldOverflow) ; } buf . put_u8 (0x05) ; buf . put_u8 (0x01) ; buf . put_u8 (* self . 0 as u8) ; Ok (3) } }
};
}
