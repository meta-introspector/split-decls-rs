// Generated macro for DyldCacheSlideInfo5 (struct)
macro_rules! Depcrate_machoDyldCacheSlideInfo5 {
() => {
// Module: crate::macho
// Provides: {"DyldCacheSlideInfo5"}
// Dependencies: {}
# [doc = " Corresponds to struct dyld_cache_slide_info5 from dyld_cache_format.h."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct DyldCacheSlideInfo5 < E : Endian > { pub version : U32 < E > , pub page_size : U32 < E > , pub page_starts_count : U32 < E > , reserved1 : [u8 ; 4] , pub value_add : U64 < E > , }
};
}
