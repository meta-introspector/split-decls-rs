// Generated macro for impl_24 (impl)
macro_rules! Depcrate_writerimpl_24 {
() => {
// Module: crate::writer
// Provides: {"impl_24"}
// Dependencies: {}
impl < D : Digest + FixedOutputReset , W : io :: Write > HashWriter < D , W > { # [doc = " Retrieve result and reset hasher instance."] pub fn finalize_reset (& mut self) -> Output < D > { Digest :: finalize_reset (& mut self . hasher) } # [doc = " Write result into provided array and reset the hasher instance."] pub fn finalize_into_reset (& mut self , out : & mut Output < D >) { Digest :: finalize_into_reset (& mut self . hasher , out) } }
};
}
