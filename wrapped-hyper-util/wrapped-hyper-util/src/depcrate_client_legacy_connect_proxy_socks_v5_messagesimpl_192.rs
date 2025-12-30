// Generated macro for impl_192 (impl)
macro_rules! Depcrate_client_legacy_connect_proxy_socks_v5_messagesimpl_192 {
() => {
// Module: crate::client::legacy::connect::proxy::socks::v5::messages
// Provides: {"impl_192"}
// Dependencies: {}
impl TryFrom < & mut BytesMut > for AuthenticationRes { type Error = ParsingError ; fn try_from (buf : & mut BytesMut) -> Result < Self , ParsingError > { if buf . remaining () < 2 { return Err (ParsingError :: Incomplete) ; } if buf . get_u8 () != 0x01 { return Err (ParsingError :: Other) ; } if buf . get_u8 () == 0 { Ok (Self (true)) } else { Ok (Self (false)) } } }
};
}
