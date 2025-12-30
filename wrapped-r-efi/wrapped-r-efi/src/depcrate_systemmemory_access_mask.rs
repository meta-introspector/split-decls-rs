// Generated macro for MEMORY_ACCESS_MASK (const)
macro_rules! Depcrate_systemMEMORY_ACCESS_MASK {
() => {
// Module: crate::system
// Provides: {"MEMORY_ACCESS_MASK"}
// Dependencies: {}
# [doc = " Mask of memory attributes that specify access protection attributes. No"] # [doc = " symbol is defined by the spec, but the attributes are annotated in the"] # [doc = " spec. Note that `MEMORY_WP` is treated as cacheability attribute, and its"] # [doc = " access protection functionality is replaced by `MEMORY_RO`."] pub const MEMORY_ACCESS_MASK : u64 = MEMORY_RP | MEMORY_XP | MEMORY_RO ;
};
}
