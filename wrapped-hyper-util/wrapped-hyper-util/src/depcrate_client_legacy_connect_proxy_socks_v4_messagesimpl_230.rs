// Generated macro for impl_230 (impl)
macro_rules! Depcrate_client_legacy_connect_proxy_socks_v4_messagesimpl_230 {
() => {
// Module: crate::client::legacy::connect::proxy::socks::v4::messages
// Provides: {"impl_230"}
// Dependencies: {}
impl Request < '_ > { pub fn write_to_buf < B : BufMut > (& self , mut buf : B) -> Result < usize , SerializeError > { match self . 0 { Address :: Socket (socket) => { if buf . remaining_mut () < 10 { return Err (SerializeError :: WouldOverflow) ; } buf . put_u8 (0x04) ; buf . put_u8 (0x01) ; buf . put_u16 (socket . port ()) ; buf . put_slice (& socket . ip () . octets ()) ; buf . put_u8 (0x00) ; buf . put_u8 (0x00) ; Ok (10) } Address :: Domain (domain , port) => { if buf . remaining_mut () < 10 + domain . len () + 1 { return Err (SerializeError :: WouldOverflow) ; } buf . put_u8 (0x04) ; buf . put_u8 (0x01) ; buf . put_u16 (* port) ; buf . put_slice (& [0x00 , 0x00 , 0x00 , 0xFF]) ; buf . put_u8 (0x00) ; buf . put_u8 (0x00) ; buf . put_slice (domain . as_bytes ()) ; buf . put_u8 (0x00) ; Ok (10 + domain . len () + 1) } } } }
};
}
