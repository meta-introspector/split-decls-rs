// Generated macro for sub_section (function)
macro_rules! Depcrate_parse_nomsub_section {
() => {
// Module: crate::parse::nom
// Provides: {"sub_section"}
// Dependencies: {}
fn sub_section < 'i > (i : & mut & 'i [u8]) -> ModalResult < Cow < 'i , BStr > , NomError < & 'i [u8] > > { let mut output = Cow :: Borrowed (Default :: default ()) ; if let Some (sub) = opt (subsection_subset) . parse_next (i) ? { output = Cow :: Borrowed (sub . as_bstr ()) ; } while let Some (sub) = opt (subsection_subset) . parse_next (i) ? { output . to_mut () . extend (sub) ; } Ok (output) }
};
}
