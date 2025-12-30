// Generated macro for take_newlines1 (function)
macro_rules! Depcrate_parse_nomtake_newlines1 {
() => {
// Module: crate::parse::nom
// Provides: {"take_newlines1"}
// Dependencies: {}
fn take_newlines1 < 'i > (i : & mut & 'i [u8]) -> ModalResult < & 'i BStr , NomError < & 'i [u8] > > { repeat (1 .. 1024 , alt (("\r\n" , "\n"))) . map (| () | ()) . take () . map (bstr :: ByteSlice :: as_bstr) . parse_next (i) }
};
}
