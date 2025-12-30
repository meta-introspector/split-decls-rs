// Generated macro for DyldCacheSlideInfo3 (struct)
macro_rules! Depcrate_machoDyldCacheSlideInfo3 {
() => {
// Module: crate::macho
// Provides: {"DyldCacheSlideInfo3"}
// Dependencies: {}
# [doc = " Corresponds to struct dyld_cache_slide_info3 from dyld_cache_format.h."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct DyldCacheSlideInfo3 < E : Endian > { pub version : U32 < E > , pub page_size : U32 < E > , pub page_starts_count : U32 < E > , reserved1 : [u8 ; 4] , pub auth_value_add : U64 < E > , }
};
}
