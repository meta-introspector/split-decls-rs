// Generated macro for impl_12 (impl)
macro_rules! Depcrate_serde_implsimpl_12 {
() => {
// Module: crate::serde_impls
// Provides: {"impl_12"}
// Dependencies: {}
impl < 'a > de :: Visitor < 'a > for Utf8PathBufVisitor { type Value = Utf8PathBuf ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a UTF-8 path string") } fn visit_str < E > (self , v : & str) -> Result < Self :: Value , E > where E : de :: Error , { Ok (v . into ()) } fn visit_string < E > (self , v : String) -> Result < Self :: Value , E > where E : de :: Error , { Ok (v . into ()) } fn visit_bytes < E > (self , v : & [u8]) -> Result < Self :: Value , E > where E : de :: Error , { std :: str :: from_utf8 (v) . map (Into :: into) . map_err (| _ | de :: Error :: invalid_value (de :: Unexpected :: Bytes (v) , & self)) } fn visit_byte_buf < E > (self , v : Vec < u8 >) -> Result < Self :: Value , E > where E : de :: Error , { String :: from_utf8 (v) . map (Into :: into) . map_err (| e | de :: Error :: invalid_value (de :: Unexpected :: Bytes (& e . into_bytes ()) , & self)) } }
};
}
