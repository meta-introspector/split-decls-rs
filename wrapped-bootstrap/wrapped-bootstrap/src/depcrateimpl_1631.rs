// Generated macro for impl_1631 (impl)
macro_rules! Depcrateimpl_1631 {
() => {
// Module: crate
// Provides: {"impl_1631"}
// Dependencies: {}
impl std :: str :: FromStr for CodegenBackendKind { type Err = & 'static str ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s . to_lowercase () . as_str () { "" => Err ("Invalid empty backend name") , "gcc" => Ok (Self :: Gcc) , "llvm" => Ok (Self :: Llvm) , "cranelift" => Ok (Self :: Cranelift) , _ => Ok (Self :: Custom (s . to_string ())) , } } }
};
}
