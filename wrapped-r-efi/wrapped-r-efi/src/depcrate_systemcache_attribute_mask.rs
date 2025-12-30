// Generated macro for CACHE_ATTRIBUTE_MASK (const)
macro_rules! Depcrate_systemCACHE_ATTRIBUTE_MASK {
() => {
// Module: crate::system
// Provides: {"CACHE_ATTRIBUTE_MASK"}
// Dependencies: {}
# [doc = " Mask of memory attributes that specify cacheability attributes. No symbol"] # [doc = " is defined by the spec, but the attributes are annotated in the spec. Note"] # [doc = " that `MEMORY_WP`, despite its name, is treated as cacheability attribute."] # [doc = " Use `MEMORY_RO` as replacement access attribute (see the spec for details)."] pub const CACHE_ATTRIBUTE_MASK : u64 = MEMORY_UC | MEMORY_WC | MEMORY_WT | MEMORY_WB | MEMORY_UCE | MEMORY_WP ;
};
}
