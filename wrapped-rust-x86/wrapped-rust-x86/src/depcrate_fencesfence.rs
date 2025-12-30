// Generated macro for sfence (function)
macro_rules! Depcrate_fencesfence {
() => {
// Module: crate::fence
// Provides: {"sfence"}
// Dependencies: {}
# [doc = " sfence -- Store Fence"] # [doc = ""] # [doc = " Orders processor execution relative to all memory stores prior to the SFENCE"] # [doc = " instruction. The processor ensures that every store prior to SFENCE is"] # [doc = " globally visible before any store after SFENCE becomes globally visible."] pub fn sfence () { unsafe { asm ! ("sfence") } ; }
};
}
