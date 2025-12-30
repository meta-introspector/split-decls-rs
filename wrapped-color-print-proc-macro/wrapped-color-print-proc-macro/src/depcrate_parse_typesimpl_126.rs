// Generated macro for impl_126 (impl)
macro_rules! Depcrate_parse_typesimpl_126 {
() => {
// Module: crate::parse::types
// Provides: {"impl_126"}
// Dependencies: {}
impl < 'a > Error < 'a > { pub fn new (input : & 'a str , code : ErrorKind , detail : Option < ErrorDetail < 'a > >) -> Self { Error { input , code , detail } } pub fn with_detail (& self , detail : ErrorDetail < 'a >) -> Self { Error { input : self . input , code : self . code , detail : Some (detail) } } }
};
}
