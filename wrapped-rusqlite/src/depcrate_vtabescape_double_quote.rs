// Generated macro for escape_double_quote (function)
macro_rules! Depcrate_vtabescape_double_quote {
() => {
// Module: crate::vtab
// Provides: {"escape_double_quote"}
// Dependencies: {}
# [doc = " Escape double-quote (`\"`) character occurrences by"] # [doc = " doubling them (`\"\"`)."] # [must_use] pub fn escape_double_quote (identifier : & str) -> Cow < '_ , str > { if identifier . contains ('"') { Owned (identifier . replace ('"' , "\"\"")) } else { Borrowed (identifier) } }
};
}
