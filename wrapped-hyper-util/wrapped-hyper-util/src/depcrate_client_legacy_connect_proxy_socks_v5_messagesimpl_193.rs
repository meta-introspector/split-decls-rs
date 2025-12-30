// Generated macro for impl_193 (impl)
macro_rules! Depcrate_client_legacy_connect_proxy_socks_v5_messagesimpl_193 {
() => {
// Module: crate::client::legacy::connect::proxy::socks::v5::messages
// Provides: {"impl_193"}
// Dependencies: {}
impl ProxyReq < '_ > { pub fn write_to_buf (& self , buf : & mut BytesMut) -> Result < usize , SerializeError > { let addr_len = match self . 0 { Address :: Socket (SocketAddr :: V4 (_)) => 1 + 4 + 2 , Address :: Socket (SocketAddr :: V6 (_)) => 1 + 16 + 2 , Address :: Domain (ref domain , _) => 1 + 1 + domain . len () + 2 , } ; if buf . capacity () - buf . len () < 3 + addr_len { return Err (SerializeError :: WouldOverflow) ; } buf . put_u8 (0x05) ; buf . put_u8 (0x01) ; buf . put_u8 (0x00) ; let _ = self . 0 . write_to_buf (buf) ; Ok (3 + addr_len) } }
};
}
