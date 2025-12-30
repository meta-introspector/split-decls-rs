// Generated macro for be_memory_inline_jit_restrict_with_witness_supported (function)
macro_rules! Depcrate_memorybe_memory_inline_jit_restrict_with_witness_supported {
() => {
// Module: crate::memory
// Provides: {"be_memory_inline_jit_restrict_with_witness_supported"}
// Dependencies: {}
# [doc = " Returns `true` iff the inlinable version of the jit_write_protect API is available."] # [inline] pub fn be_memory_inline_jit_restrict_with_witness_supported () -> bool { extern "C" { fn be_memory_inline_jit_restrict_with_witness_supported () -> c_int ; } unsafe { be_memory_inline_jit_restrict_with_witness_supported () != 0 } }
};
}
