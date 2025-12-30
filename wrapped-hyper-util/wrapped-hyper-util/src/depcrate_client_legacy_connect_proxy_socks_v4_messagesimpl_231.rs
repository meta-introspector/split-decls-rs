// Generated macro for impl_231 (impl)
macro_rules! Depcrate_client_legacy_connect_proxy_socks_v4_messagesimpl_231 {
() => {
// Module: crate::client::legacy::connect::proxy::socks::v4::messages
// Provides: {"impl_231"}
// Dependencies: {}
impl TryFrom < & mut BytesMut > for Response { type Error = ParsingError ; fn try_from (buf : & mut BytesMut) -> Result < Self , Self :: Error > { if buf . remaining () < 8 { return Err (ParsingError :: Incomplete) ; } if buf . get_u8 () != 0x00 { return Err (ParsingError :: Other) ; } let status = buf . get_u8 () . try_into () ? ; let _addr = { let port = buf . get_u16 () ; let mut ip = [0 ; 4] ; buf . copy_to_slice (& mut ip) ; SocketAddrV4 :: new (ip . into () , port) } ; Ok (Self (status)) } }
};
}
