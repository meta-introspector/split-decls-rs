// Generated macro for impl_117 (impl)
macro_rules! Depcrate_gv_parser_astimpl_117 {
() => {
// Module: crate::gv::parser::ast
// Provides: {"impl_117"}
// Dependencies: {}
impl AttributeList { pub fn new () -> Self { Self { list : Vec :: new () } } pub fn add_attr (& mut self , from : & str , to : & str) { self . list . push ((from . to_string () , to . to_string ())) ; } pub fn iter (& self) -> std :: slice :: Iter < (String , String) > { self . list . iter () } }
};
}
