// Generated macro for LIMB_BITS (const)
macro_rules! Depcrate_bigintLIMB_BITS {
() => {
// Module: crate::bigint
// Provides: {"LIMB_BITS"}
// Dependencies: {}
# [cfg (not (all (target_pointer_width = "64" , not (target_arch = "sparc"))))] pub const LIMB_BITS : usize = 32 ;
};
}
