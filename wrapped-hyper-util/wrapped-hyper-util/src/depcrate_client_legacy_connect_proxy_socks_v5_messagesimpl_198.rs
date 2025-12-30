// Generated macro for impl_198 (impl)
macro_rules! Depcrate_client_legacy_connect_proxy_socks_v5_messagesimpl_198 {
() => {
// Module: crate::client::legacy::connect::proxy::socks::v5::messages
// Provides: {"impl_198"}
// Dependencies: {}
impl TryFrom < u8 > for AuthMethod { type Error = ParsingError ; fn try_from (byte : u8) -> Result < Self , Self :: Error > { Ok (match byte { 0x00 => Self :: NoAuth , 0x02 => Self :: UserPass , 0xFF => Self :: NoneAcceptable , _ => return Err (ParsingError :: Other) , }) } }
};
}
