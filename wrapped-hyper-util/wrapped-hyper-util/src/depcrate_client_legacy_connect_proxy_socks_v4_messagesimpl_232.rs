// Generated macro for impl_232 (impl)
macro_rules! Depcrate_client_legacy_connect_proxy_socks_v4_messagesimpl_232 {
() => {
// Module: crate::client::legacy::connect::proxy::socks::v4::messages
// Provides: {"impl_232"}
// Dependencies: {}
impl TryFrom < u8 > for Status { type Error = ParsingError ; fn try_from (byte : u8) -> Result < Self , Self :: Error > { Ok (match byte { 90 => Self :: Success , 91 => Self :: Failed , 92 => Self :: IdentFailure , 93 => Self :: IdentMismatch , _ => return Err (ParsingError :: Other) , }) } }
};
}
