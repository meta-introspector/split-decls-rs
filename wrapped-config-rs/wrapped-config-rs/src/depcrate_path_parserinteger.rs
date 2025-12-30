// Generated macro for integer (function)
macro_rules! Depcrate_path_parserinteger {
() => {
// Module: crate::path::parser
// Provides: {"integer"}
// Dependencies: {}
fn integer (i : & mut & str) -> ModalResult < isize > { seq ! (_ : space0 , (opt ('-') , digit1) . take () . try_map (FromStr :: from_str) , _ : space0) . context (StrContext :: Expected (StrContextValue :: Description ("integer" ,))) . map (| (i ,) | i) . parse_next (i) }
};
}
