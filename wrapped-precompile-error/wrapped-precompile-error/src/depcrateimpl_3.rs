// Generated macro for impl_3 (impl)
macro_rules! Depcrateimpl_3 {
() => {
// Module: crate
// Provides: {"impl_3"}
// Dependencies: {}
impl num_traits :: FromPrimitive for PrecompileError { # [inline] fn from_i64 (n : i64) -> Option < Self > { if n == PrecompileError :: InvalidPublicKey as i64 { Some (PrecompileError :: InvalidPublicKey) } else if n == PrecompileError :: InvalidRecoveryId as i64 { Some (PrecompileError :: InvalidRecoveryId) } else if n == PrecompileError :: InvalidSignature as i64 { Some (PrecompileError :: InvalidSignature) } else if n == PrecompileError :: InvalidDataOffsets as i64 { Some (PrecompileError :: InvalidDataOffsets) } else if n == PrecompileError :: InvalidInstructionDataSize as i64 { Some (PrecompileError :: InvalidInstructionDataSize) } else { None } } # [inline] fn from_u64 (n : u64) -> Option < Self > { Self :: from_i64 (n as i64) } }
};
}
