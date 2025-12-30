// Generated macro for macro_253 (macro)
macro_rules! Depcrate_symmetric_keymacro_253 {
() => {
// Module: crate::symmetric_key
// Provides: {"macro_253"}
// Dependencies: {}
impl_str_enum ! (# [doc = " The `PINUsageMode` type is defined in [RFC 6031 Section 3.3.5]"] # [doc = ""] # [doc = " ```text"] # [doc = "   PINUsageMode ::= UTF8String (\"Local\" | \"Prepend\" | \"Append\" |"] # [doc = "                     \"Algorithmic\")"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6031 Section 3.3.5]: https://datatracker.ietf.org/doc/html/rfc6031#section-3.3.5"] # [derive (Copy , Clone , PartialEq , Eq)] pub enum PINUsageMode { # [doc = " \"Local\""] Local => "Local" , # [doc = " \"Prepend\""] Prepend => "Prepend" , # [doc = " \"Append\""] Append => "Append" , # [doc = " \"Algorithmic\""] Algorithmic => "Algorithmic" , }) ;
};
}
