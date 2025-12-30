// Generated macro for impl_75 (impl)
macro_rules! Depcrateimpl_75 {
() => {
// Module: crate
// Provides: {"impl_75"}
// Dependencies: {}
impl digest :: FixedOutputDirty for Groestl384 { type OutputSize = U48 ; fn finalize_into_dirty (& mut self , out : & mut DGenericArray < u8 , Self :: OutputSize >) { let result = self . 0 . finalize_dirty () ; for (out , & input) in out . chunks_exact_mut (8) . zip (& result [10 ..]) { out . copy_from_slice (& input . to_le_bytes ()) ; } } }
};
}
