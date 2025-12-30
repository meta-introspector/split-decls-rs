// Generated macro for impl_14 (impl)
macro_rules! Depcrate_readerimpl_14 {
() => {
// Module: crate::reader
// Provides: {"impl_14"}
// Dependencies: {}
impl < D : Digest + FixedOutputReset , R : io :: Read > HashReader < D , R > { # [doc = " Retrieve result and reset hasher instance."] pub fn finalize_reset (& mut self) -> Output < D > { Digest :: finalize_reset (& mut self . hasher) } # [doc = " Rrite result into provided array and reset the hasher instance."] pub fn finalize_into_reset (& mut self , out : & mut Output < D >) { Digest :: finalize_into_reset (& mut self . hasher , out) } }
};
}
