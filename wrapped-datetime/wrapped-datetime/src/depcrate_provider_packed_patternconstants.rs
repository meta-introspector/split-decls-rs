// Generated macro for constants (module)
macro_rules! Depcrate_provider_packed_patternconstants {
() => {
// Module: crate::provider::packed_pattern
// Provides: {"constants"}
// Dependencies: {}
mod constants { # [doc = " Value when standard long, medium, and short are all the same"] pub (super) const LMS : u32 = 0 ; # [doc = " Value when standard medium is the same as short but not long"] pub (super) const L_MS : u32 = 1 ; # [doc = " Value when standard medium is the same as long but not short"] pub (super) const LM_S : u32 = 2 ; # [doc = " Bit that indicates that standard medium differs from standard long"] pub (super) const M_DIFFERS : u32 = 0x1 ; # [doc = " Bit that indicates that standard short differs from standard medium"] pub (super) const S_DIFFERS : u32 = 0x2 ; # [doc = " Bitmask over all LMS values"] pub (super) const LMS_MASK : u32 = 0x3 ; # [doc = " Bit that indicates whether there are per-cell chunks"] pub (super) const Q_BIT : u32 = 0x4 ; # [doc = " A mask applied to individual chunks (the largest possible chunk)"] pub (super) const CHUNK_MASK : u32 = 0x7 ; }
};
}
