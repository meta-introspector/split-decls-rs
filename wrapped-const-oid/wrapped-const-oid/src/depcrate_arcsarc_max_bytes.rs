// Generated macro for ARC_MAX_BYTES (const)
macro_rules! Depcrate_arcsARC_MAX_BYTES {
() => {
// Module: crate::arcs
// Provides: {"ARC_MAX_BYTES"}
// Dependencies: {}
# [doc = " Maximum number of bytes supported in an arc."] # [doc = ""] # [doc = " Note that OIDs are base 128 encoded (with continuation bits), so we must consider how many bytes"] # [doc = " are required when each byte can only represent 7-bits of the input."] const ARC_MAX_BYTES : usize = (Arc :: BITS as usize) . div_ceil (7) ;
};
}
