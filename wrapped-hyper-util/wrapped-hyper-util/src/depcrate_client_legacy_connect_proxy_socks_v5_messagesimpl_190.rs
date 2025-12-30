// Generated macro for impl_190 (impl)
macro_rules! Depcrate_client_legacy_connect_proxy_socks_v5_messagesimpl_190 {
() => {
// Module: crate::client::legacy::connect::proxy::socks::v5::messages
// Provides: {"impl_190"}
// Dependencies: {}
impl TryFrom < & mut BytesMut > for NegotiationRes { type Error = ParsingError ; fn try_from (buf : & mut BytesMut) -> Result < Self , ParsingError > { if buf . remaining () < 2 { return Err (ParsingError :: Incomplete) ; } if buf . get_u8 () != 0x05 { return Err (ParsingError :: Other) ; } let method = buf . get_u8 () . try_into () ? ; Ok (Self (method)) } }
};
}
