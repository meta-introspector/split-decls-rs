// Generated macro for impl_191 (impl)
macro_rules! Depcrate_client_legacy_connect_proxy_socks_v5_messagesimpl_191 {
() => {
// Module: crate::client::legacy::connect::proxy::socks::v5::messages
// Provides: {"impl_191"}
// Dependencies: {}
impl AuthenticationReq < '_ > { pub fn write_to_buf (& self , buf : & mut BytesMut) -> Result < usize , SerializeError > { if buf . capacity () - buf . len () < 3 + self . 0 . len () + self . 1 . len () { return Err (SerializeError :: WouldOverflow) ; } buf . put_u8 (0x01) ; buf . put_u8 (self . 0 . len () as u8) ; buf . put_slice (self . 0 . as_bytes ()) ; buf . put_u8 (self . 1 . len () as u8) ; buf . put_slice (self . 1 . as_bytes ()) ; Ok (3 + self . 0 . len () + self . 1 . len ()) } }
};
}
