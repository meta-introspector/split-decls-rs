// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl < 'ast > Visit < 'ast > for TokenVisitor { fn visit_ident (& mut self , ident : & 'ast Ident) { let name = ident . to_string () ; if ! ["fn" , "let" , "mut" , "if" , "else" , "match" , "for" , "while" , "loop" , "return" , "Ok" , "Err"] . contains (& name . as_str ()) { self . uses . insert (name) ; } } }
};
}
