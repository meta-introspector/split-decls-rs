// Generated macro for hex_hash (function)
macro_rules! Depcrate_parsehex_hash {
() => {
// Module: crate::parse
// Provides: {"hex_hash"}
// Dependencies: {}
pub fn hex_hash < 'a , E : ParserError < & 'a [u8] > > (i : & mut & 'a [u8]) -> ModalResult < & 'a BStr , E > { take_while (gix_hash :: Kind :: shortest () . len_in_hex () ..= gix_hash :: Kind :: longest () . len_in_hex () , is_hex_digit_lc ,) . map (ByteSlice :: as_bstr) . parse_next (i) }
};
}
