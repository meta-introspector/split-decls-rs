// Generated macro for impl_259 (impl)
macro_rules! Depcrate_parse_section_headerimpl_259 {
() => {
// Module: crate::parse::section::header
// Provides: {"impl_259"}
// Dependencies: {}
impl < 'a > Header < 'a > { # [doc = " Instantiate a new header either with a section `name`, e.g. \"core\" serializing to `[\"core\"]`"] # [doc = " or `[remote \"origin\"]` for `subsection` being \"origin\" and `name` being \"remote\"."] pub fn new (name : impl Into < Cow < 'a , str > > , subsection : impl Into < Option < Cow < 'a , BStr > > > ,) -> Result < Header < 'a > , Error > { let name = Name (validated_name (into_cow_bstr (name . into ())) ?) ; if let Some (subsection_name) = subsection . into () { Ok (Header { name , separator : Some (Cow :: Borrowed (" " . into ())) , subsection_name : Some (validated_subsection (subsection_name) ?) , }) } else { Ok (Header { name , separator : None , subsection_name : None , }) } } }
};
}
