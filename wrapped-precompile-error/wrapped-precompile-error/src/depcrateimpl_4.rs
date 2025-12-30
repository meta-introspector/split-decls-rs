// Generated macro for impl_4 (impl)
macro_rules! Depcrateimpl_4 {
() => {
// Module: crate
// Provides: {"impl_4"}
// Dependencies: {}
impl num_traits :: ToPrimitive for PrecompileError { # [inline] fn to_i64 (& self) -> Option < i64 > { Some (match * self { PrecompileError :: InvalidPublicKey => PrecompileError :: InvalidPublicKey as i64 , PrecompileError :: InvalidRecoveryId => PrecompileError :: InvalidRecoveryId as i64 , PrecompileError :: InvalidSignature => PrecompileError :: InvalidSignature as i64 , PrecompileError :: InvalidDataOffsets => PrecompileError :: InvalidDataOffsets as i64 , PrecompileError :: InvalidInstructionDataSize => { PrecompileError :: InvalidInstructionDataSize as i64 } }) } # [inline] fn to_u64 (& self) -> Option < u64 > { self . to_i64 () . map (| x | x as u64) } }
};
}
