// Generated macro for lfence (function)
macro_rules! Depcrate_fencelfence {
() => {
// Module: crate::fence
// Provides: {"lfence"}
// Dependencies: {}
# [doc = " lfence -- Load Fence"] # [doc = ""] # [doc = " Performs a serializing operation on all load-from-memory instructions that"] # [doc = " were issued prior the LFENCE instruction. Specifically, LFENCE does not"] # [doc = " execute until all prior instructions have completed locally, and no later"] # [doc = " instruction begins execution until LFENCE completes."] pub fn lfence () { unsafe { asm ! ("lfence") } ; }
};
}
