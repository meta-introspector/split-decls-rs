// Generated macro for impl_194 (impl)
macro_rules! Depcrate_client_legacy_connect_proxy_socks_v5_messagesimpl_194 {
() => {
// Module: crate::client::legacy::connect::proxy::socks::v5::messages
// Provides: {"impl_194"}
// Dependencies: {}
impl TryFrom < & mut BytesMut > for ProxyRes { type Error = ParsingError ; fn try_from (buf : & mut BytesMut) -> Result < Self , ParsingError > { if buf . remaining () < 3 { return Err (ParsingError :: Incomplete) ; } if buf . get_u8 () != 0x05 { return Err (ParsingError :: Other) ; } let status = buf . get_u8 () . try_into () ? ; if buf . get_u8 () != 0x00 { return Err (ParsingError :: Other) ; } Address :: try_from (buf) ? ; Ok (Self (status)) } }
};
}
