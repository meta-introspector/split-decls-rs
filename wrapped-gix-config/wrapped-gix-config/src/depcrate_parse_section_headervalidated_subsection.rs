// Generated macro for validated_subsection (function)
macro_rules! Depcrate_parse_section_headervalidated_subsection {
() => {
// Module: crate::parse::section::header
// Provides: {"validated_subsection"}
// Dependencies: {}
fn validated_subsection (name : Cow < '_ , BStr >) -> Result < Cow < '_ , BStr > , Error > { is_valid_subsection (name . as_ref ()) . then_some (name) . ok_or (Error :: InvalidSubSection) }
};
}
