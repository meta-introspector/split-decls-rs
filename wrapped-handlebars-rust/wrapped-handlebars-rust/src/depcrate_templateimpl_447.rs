// Generated macro for impl_447 (impl)
macro_rules! Depcrate_templateimpl_447 {
() => {
// Module: crate::template
// Provides: {"impl_447"}
// Dependencies: {}
impl Parameter { pub fn as_name (& self) -> Option < & str > { match self { Parameter :: Name (ref n) => Some (n) , Parameter :: Path (ref p) => Some (p . raw ()) , _ => None , } } pub fn parse (s : & str) -> Result < Parameter , TemplateError > { let parser = HandlebarsParser :: parse (Rule :: parameter , s) . map_err (| _ | TemplateError :: of (TemplateErrorReason :: InvalidParam (s . to_owned ()))) ? ; let mut it = parser . flatten () . peekable () ; Template :: parse_param (s , & mut it , s . len () - 1) } fn debug_name (& self) -> String { if let Some (name) = self . as_name () { name . to_owned () } else { format ! ("{self:?}") } } }
};
}
