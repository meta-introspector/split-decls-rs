// Generated macro for impl_69 (impl)
macro_rules! Depcrateimpl_69 {
() => {
// Module: crate
// Provides: {"impl_69"}
// Dependencies: {}
impl digest :: FixedOutputDirty for Groestl224 { type OutputSize = U28 ; fn finalize_into_dirty (& mut self , out : & mut DGenericArray < u8 , Self :: OutputSize >) { let result = self . 0 . finalize_dirty () ; out [.. 4] . copy_from_slice (& ((result [4] >> 32) as u32) . to_le_bytes ()) ; for (out , & input) in out [4 ..] . chunks_exact_mut (8) . zip (& result [5 .. 8]) { out . copy_from_slice (& input . to_le_bytes ()) ; } } }
};
}
