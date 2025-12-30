// Generated macro for impl_196 (impl)
macro_rules! Depcrate_client_legacy_connect_proxy_socks_v5_messagesimpl_196 {
() => {
// Module: crate::client::legacy::connect::proxy::socks::v5::messages
// Provides: {"impl_196"}
// Dependencies: {}
impl TryFrom < & mut BytesMut > for Address { type Error = ParsingError ; fn try_from (buf : & mut BytesMut) -> Result < Self , Self :: Error > { if buf . remaining () < 2 { return Err (ParsingError :: Incomplete) ; } Ok (match buf . get_u8 () { 0x01 => { let mut ip = [0 ; 4] ; if buf . remaining () < 6 { return Err (ParsingError :: Incomplete) ; } buf . copy_to_slice (& mut ip) ; let port = buf . get_u16 () ; Self :: Socket (SocketAddr :: new (ip . into () , port)) } 0x03 => { let len = buf . get_u8 () ; if len == 0 { return Err (ParsingError :: Other) ; } else if buf . remaining () < (len as usize) + 2 { return Err (ParsingError :: Incomplete) ; } let domain = std :: str :: from_utf8 (& buf [.. len as usize]) . map_err (| _ | ParsingError :: Other) ? . to_string () ; let port = buf . get_u16 () ; Self :: Domain (domain , port) } 0x04 => { let mut ip = [0 ; 16] ; if buf . remaining () < 18 { return Err (ParsingError :: Incomplete) ; } buf . copy_to_slice (& mut ip) ; let port = buf . get_u16 () ; Self :: Socket (SocketAddr :: new (ip . into () , port)) } _ => return Err (ParsingError :: Other) , }) } }
};
}
