// Generated macro for take_spaces1 (function)
macro_rules! Depcrate_parse_nomtake_spaces1 {
() => {
// Module: crate::parse::nom
// Provides: {"take_spaces1"}
// Dependencies: {}
fn take_spaces1 < 'i > (i : & mut & 'i [u8]) -> ModalResult < & 'i BStr , NomError < & 'i [u8] > > { take_while (1 .. , winnow :: stream :: AsChar :: is_space) . map (bstr :: ByteSlice :: as_bstr) . parse_next (i) }
};
}
