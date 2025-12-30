// Generated macro for try_percent_encode_iri_inline (function)
macro_rules! Depcrate_converttry_percent_encode_iri_inline {
() => {
// Module: crate::convert
// Provides: {"try_percent_encode_iri_inline"}
// Dependencies: {}
# [doc = " Percent-encodes the given IRI using the given buffer."] # [cfg (feature = "alloc")] pub (crate) fn try_percent_encode_iri_inline (iri : & mut String ,) -> Result < () , alloc :: collections :: TryReserveError > { let num_nonascii = count_nonascii (iri) ; if num_nonascii == 0 { return Ok (()) ; } let additional = num_nonascii * 2 ; iri . try_reserve (additional) ? ; let src_len = iri . len () ; let mut buf = core :: mem :: take (iri) . into_bytes () ; buf . extend (core :: iter :: repeat (b'\0') . take (additional)) ; let mut dest_end = buf . len () ; let mut src_end = src_len ; let mut rest_nonascii = num_nonascii ; while rest_nonascii > 0 { debug_assert ! (src_end > 0 , "[validity] the source position should not overrun") ; debug_assert ! (dest_end > 0 , "[validity] the destination position should not overrun") ; src_end -= 1 ; dest_end -= 1 ; let byte = buf [src_end] ; if byte . is_ascii () { buf [dest_end] = byte ; } else { dest_end -= 2 ; buf [dest_end] = b'%' ; let upper = byte >> 4 ; let lower = byte & 0b1111 ; buf [dest_end + 1] = HEXDIGITS [usize :: from (upper)] ; buf [dest_end + 2] = HEXDIGITS [usize :: from (lower)] ; rest_nonascii -= 1 ; } } let s = String :: from_utf8 (buf) . expect ("[consistency] the encoding result is an ASCII string") ; * iri = s ; Ok (()) }
};
}
