// Generated macro for impl_126 (impl)
macro_rules! Depcrateimpl_126 {
() => {
// Module: crate
// Provides: {"impl_126"}
// Dependencies: {}
impl Drop for FixedBitSet { fn drop (& mut self) { drop (unsafe { Vec :: from_raw_parts (self . data . as_ptr () , self . simd_block_len () , self . capacity) }) ; } }
};
}
