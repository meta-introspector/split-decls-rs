// Generated macro for EvexContext (enum)
macro_rules! Depcrate_isa_x64_encoding_evexEvexContext {
() => {
// Module: crate::isa::x64::encoding::evex
// Provides: {"EvexContext"}
// Dependencies: {}
# [doc = " Defines the EVEX context for the `L'`, `L`, and `b` bits (bits 6:4 of EVEX P2 byte). Table 2-36 in"] # [doc = " section 2.6.10 (Intel Software Development Manual, volume 2A) describes how these bits can be"] # [doc = " used together for certain classes of instructions; i.e., special care should be taken to ensure"] # [doc = " that instructions use an applicable correct `EvexContext`. Table 2-39 contains cases where"] # [doc = " opcodes can result in an #UD."] # [allow (dead_code , missing_docs)] pub enum EvexContext { RoundingRegToRegFP { rc : EvexRoundingControl , } , NoRoundingFP { sae : bool , length : EvexVectorLength , } , MemoryOp { broadcast : bool , length : EvexVectorLength , } , Other { length : EvexVectorLength , } , }
};
}
