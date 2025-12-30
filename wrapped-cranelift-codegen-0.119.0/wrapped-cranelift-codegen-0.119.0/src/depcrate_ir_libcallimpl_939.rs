// Generated macro for impl_939 (impl)
macro_rules! Depcrate_ir_libcallimpl_939 {
() => {
// Module: crate::ir::libcall
// Provides: {"impl_939"}
// Dependencies: {}
impl FromStr for LibCall { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "Probestack" => Ok (Self :: Probestack) , "CeilF32" => Ok (Self :: CeilF32) , "CeilF64" => Ok (Self :: CeilF64) , "FloorF32" => Ok (Self :: FloorF32) , "FloorF64" => Ok (Self :: FloorF64) , "TruncF32" => Ok (Self :: TruncF32) , "TruncF64" => Ok (Self :: TruncF64) , "NearestF32" => Ok (Self :: NearestF32) , "NearestF64" => Ok (Self :: NearestF64) , "FmaF32" => Ok (Self :: FmaF32) , "FmaF64" => Ok (Self :: FmaF64) , "Memcpy" => Ok (Self :: Memcpy) , "Memset" => Ok (Self :: Memset) , "Memmove" => Ok (Self :: Memmove) , "Memcmp" => Ok (Self :: Memcmp) , "ElfTlsGetAddr" => Ok (Self :: ElfTlsGetAddr) , "ElfTlsGetOffset" => Ok (Self :: ElfTlsGetOffset) , "X86Pshufb" => Ok (Self :: X86Pshufb) , _ => Err (()) , } } }
};
}
