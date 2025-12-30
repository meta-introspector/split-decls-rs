// Generated macro for impl_273 (impl)
macro_rules! Depcrate_parse_section_unvalidatedimpl_273 {
() => {
// Module: crate::parse::section::unvalidated
// Provides: {"impl_273"}
// Dependencies: {}
impl < 'a > Key < 'a > { # [doc = " Parse `input` like `remote.origin` or `core` as a `Key` to make its section specific fields available,"] # [doc = " or `None` if there were not one or two tokens separated by `.`."] # [doc = " Note that `input` isn't validated, and is `str` as ascii is a subset of UTF-8 which is required for any valid keys."] pub fn parse (input : impl Into < & 'a BStr >) -> Option < Self > { let input = input . into () ; let mut tokens = input . splitn (2 , | b | * b == b'.') ; Some (Key { section_name : tokens . next () ? . to_str () . ok () ? , subsection_name : tokens . next () . map (Into :: into) , }) } }
};
}
