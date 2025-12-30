// Generated macro for Wide (type)
macro_rules! Depcrate_bigintWide {
() => {
// Module: crate::bigint
// Provides: {"Wide"}
// Dependencies: {}
# [cfg (not (all (target_pointer_width = "64" , not (target_arch = "sparc"))))] pub type Wide = u64 ;
};
}
