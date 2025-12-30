// Generated macro for comment (function)
macro_rules! Depcrate_parse_nomcomment {
() => {
// Module: crate::parse::nom
// Provides: {"comment"}
// Dependencies: {}
fn comment < 'i > (i : & mut & 'i [u8]) -> ModalResult < Comment < 'i > , NomError < & 'i [u8] > > { (one_of ([';' , '#']) , take_till (0 .. , | c | c == b'\n') . map (| text : & [u8] | Cow :: Borrowed (text . as_bstr ())) ,) . map (| (tag , text) | Comment { tag , text }) . parse_next (i) }
};
}
