// Generated macro for RexFlags (struct)
macro_rules! Depcrate_isa_x64_encoding_rexRexFlags {
() => {
// Module: crate::isa::x64::encoding::rex
// Provides: {"RexFlags"}
// Dependencies: {}
# [doc = " A small bit field to record a REX prefix specification:"] # [doc = " - bit 0 set to 1 indicates REX.W must be 0 (cleared)."] # [doc = " - bit 1 set to 1 indicates the REX prefix must always be emitted."] # [repr (transparent)] # [derive (Clone , Copy)] pub struct RexFlags (u8) ;
};
}
