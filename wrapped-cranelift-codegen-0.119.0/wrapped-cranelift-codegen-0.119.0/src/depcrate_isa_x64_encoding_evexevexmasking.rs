// Generated macro for EvexMasking (enum)
macro_rules! Depcrate_isa_x64_encoding_evexEvexMasking {
() => {
// Module: crate::isa::x64::encoding::evex
// Provides: {"EvexMasking"}
// Dependencies: {}
# [doc = " Defines the EVEX masking behavior; masking support is described in section 2.6.4 of the Intel"] # [doc = " Software Development Manual, volume 2A."] # [allow (dead_code , missing_docs)] pub enum EvexMasking { None , Merging { k : u8 } , Zeroing { k : u8 } , }
};
}
