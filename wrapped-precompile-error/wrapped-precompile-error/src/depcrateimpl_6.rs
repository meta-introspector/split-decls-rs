// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl fmt :: Display for PrecompileError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { PrecompileError :: InvalidPublicKey => f . write_str ("public key is not valid") , PrecompileError :: InvalidRecoveryId => f . write_str ("id is not valid") , PrecompileError :: InvalidSignature => f . write_str ("signature is not valid") , PrecompileError :: InvalidDataOffsets => f . write_str ("offset not valid") , PrecompileError :: InvalidInstructionDataSize => { f . write_str ("instruction is incorrect size") } } } }
};
}
