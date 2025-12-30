// Generated macro for impl_32 (impl)
macro_rules! Depcrate_commonimpl_32 {
() => {
// Module: crate::common
// Provides: {"impl_32"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a str > for CodegenBackend { type Error = & 'static str ; fn try_from (value : & 'a str) -> Result < Self , Self :: Error > { match value . to_lowercase () . as_str () { "cranelift" => Ok (Self :: Cranelift) , "gcc" => Ok (Self :: Gcc) , "llvm" => Ok (Self :: Llvm) , _ => Err ("unknown backend") , } } }
};
}
