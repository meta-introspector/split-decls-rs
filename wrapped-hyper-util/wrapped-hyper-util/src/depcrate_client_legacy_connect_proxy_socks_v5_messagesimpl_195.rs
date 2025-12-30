// Generated macro for impl_195 (impl)
macro_rules! Depcrate_client_legacy_connect_proxy_socks_v5_messagesimpl_195 {
() => {
// Module: crate::client::legacy::connect::proxy::socks::v5::messages
// Provides: {"impl_195"}
// Dependencies: {}
impl Address { pub fn write_to_buf (& self , buf : & mut BytesMut) -> Result < usize , SerializeError > { match self { Self :: Socket (SocketAddr :: V4 (v4)) => { if buf . capacity () - buf . len () < 1 + 4 + 2 { return Err (SerializeError :: WouldOverflow) ; } buf . put_u8 (0x01) ; buf . put_slice (& v4 . ip () . octets ()) ; buf . put_u16 (v4 . port ()) ; Ok (7) } Self :: Socket (SocketAddr :: V6 (v6)) => { if buf . capacity () - buf . len () < 1 + 16 + 2 { return Err (SerializeError :: WouldOverflow) ; } buf . put_u8 (0x04) ; buf . put_slice (& v6 . ip () . octets ()) ; buf . put_u16 (v6 . port ()) ; Ok (19) } Self :: Domain (domain , port) => { if buf . capacity () - buf . len () < 1 + 1 + domain . len () + 2 { return Err (SerializeError :: WouldOverflow) ; } buf . put_u8 (0x03) ; buf . put_u8 (domain . len () as u8) ; buf . put_slice (domain . as_bytes ()) ; buf . put_u16 (* port) ; Ok (4 + domain . len ()) } } } }
};
}
