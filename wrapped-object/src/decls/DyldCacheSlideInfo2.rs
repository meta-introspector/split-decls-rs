macro_rules! deps {
    () => {
        U32!();
        Endian!();
        U64!();
    };
}

macro_rules! DyldCacheSlideInfo2 {
    () => {
        deps!();
        # [doc = " Corresponds to struct dyld_cache_slide_info2 from dyld_cache_format.h."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct DyldCacheSlideInfo2 < E : Endian > { pub version : U32 < E > , pub page_size : U32 < E > , pub page_starts_offset : U32 < E > , pub page_starts_count : U32 < E > , pub page_extras_offset : U32 < E > , pub page_extras_count : U32 < E > , pub delta_mask : U64 < E > , pub value_add : U64 < E > , }
    };
}

DyldCacheSlideInfo2!();