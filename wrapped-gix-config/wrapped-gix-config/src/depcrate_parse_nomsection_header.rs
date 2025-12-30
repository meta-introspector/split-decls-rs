// Generated macro for section_header (function)
macro_rules! Depcrate_parse_nomsection_header {
() => {
// Module: crate::parse::nom
// Provides: {"section_header"}
// Dependencies: {}
fn section_header < 'i > (i : & mut & 'i [u8]) -> ModalResult < section :: Header < 'i > , NomError < & 'i [u8] > > { let name = preceded ('[' , take_while (1 .. , is_section_char) . map (bstr :: ByteSlice :: as_bstr)) . parse_next (i) ? ; if opt (one_of :: < _ , _ , ErrMode < NomError < & [u8] > > > (']')) . parse_next (i) ? . is_some () { let header = match memchr :: memrchr (b'.' , name . as_bytes ()) { Some (index) => section :: Header { name : section :: Name (Cow :: Borrowed (name [.. index] . as_bstr ())) , separator : name . get (index ..= index) . map (| s | Cow :: Borrowed (s . as_bstr ())) , subsection_name : name . get (index + 1 ..) . map (| s | Cow :: Borrowed (s . as_bstr ())) , } , None => section :: Header { name : section :: Name (Cow :: Borrowed (name . as_bstr ())) , separator : None , subsection_name : None , } , } ; if header . name . is_empty () { return Err (winnow :: error :: ErrMode :: from_input (i)) ; } return Ok (header) ; } (take_spaces1 , delimited ('"' , opt (sub_section) , "\"]")) . map (| (whitespace , subsection_name) | section :: Header { name : section :: Name (Cow :: Borrowed (name)) , separator : Some (Cow :: Borrowed (whitespace)) , subsection_name , }) . parse_next (i) }
};
}
