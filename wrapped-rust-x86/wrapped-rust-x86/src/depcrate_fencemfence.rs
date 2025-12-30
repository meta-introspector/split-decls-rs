// Generated macro for mfence (function)
macro_rules! Depcrate_fencemfence {
() => {
// Module: crate::fence
// Provides: {"mfence"}
// Dependencies: {}
# [doc = " mfence -- Memory Fence"] # [doc = ""] # [doc = " Performs a serializing operation on all load-from-memory and store-to-memory"] # [doc = " instructions that were issued prior the MFENCE instruction."] pub fn mfence () { unsafe { asm ! ("mfence") } ; }
};
}
