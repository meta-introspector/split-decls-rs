// Generated macro for unprotect_header (function)
macro_rules! Depcrate_connection_packet_cryptounprotect_header {
() => {
// Module: crate::connection::packet_crypto
// Provides: {"unprotect_header"}
// Dependencies: {}
# [doc = " Removes header protection of a packet, or returns `None` if the packet was dropped"] pub (super) fn unprotect_header (partial_decode : PartialDecode , spaces : & [PacketSpace ; 3] , zero_rtt_crypto : Option < & ZeroRttCrypto > , stateless_reset_token : Option < ResetToken > ,) -> Option < UnprotectHeaderResult > { let header_crypto = if partial_decode . is_0rtt () { if let Some (crypto) = zero_rtt_crypto { Some (& * crypto . header) } else { debug ! ("dropping unexpected 0-RTT packet") ; return None ; } } else if let Some (space) = partial_decode . space () { if let Some (ref crypto) = spaces [space] . crypto { Some (& * crypto . header . remote) } else { debug ! ("discarding unexpected {:?} packet ({} bytes)" , space , partial_decode . len () ,) ; return None ; } } else { None } ; let packet = partial_decode . data () ; let stateless_reset = packet . len () >= RESET_TOKEN_SIZE + 5 && stateless_reset_token . as_deref () == Some (& packet [packet . len () - RESET_TOKEN_SIZE ..]) ; match partial_decode . finish (header_crypto) { Ok (packet) => Some (UnprotectHeaderResult { packet : Some (packet) , stateless_reset , }) , Err (_) if stateless_reset => Some (UnprotectHeaderResult { packet : None , stateless_reset : true , }) , Err (e) => { trace ! ("unable to complete packet decoding: {}" , e) ; None } } }
};
}
