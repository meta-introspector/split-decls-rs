// Generated macro for impl_197 (impl)
macro_rules! Depcrate_client_legacy_connect_proxy_socks_v5_messagesimpl_197 {
() => {
// Module: crate::client::legacy::connect::proxy::socks::v5::messages
// Provides: {"impl_197"}
// Dependencies: {}
impl TryFrom < u8 > for Status { type Error = ParsingError ; fn try_from (byte : u8) -> Result < Self , Self :: Error > { Ok (match byte { 0x00 => Self :: Success , 0x01 => Self :: GeneralServerFailure , 0x02 => Self :: ConnectionNotAllowed , 0x03 => Self :: NetworkUnreachable , 0x04 => Self :: HostUnreachable , 0x05 => Self :: ConnectionRefused , 0x06 => Self :: TtlExpired , 0x07 => Self :: CommandNotSupported , 0x08 => Self :: AddressTypeNotSupported , _ => return Err (ParsingError :: Other) , }) } }
};
}
