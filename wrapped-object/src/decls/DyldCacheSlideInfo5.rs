macro_rules! deps {
    () => {
        Endian!();
        U64!();
        U32!();
    };
}

macro_rules! DyldCacheSlideInfo5 {
    () => {
        deps!();
        # [doc = " Corresponds to struct dyld_cache_slide_info5 from dyld_cache_format.h."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct DyldCacheSlideInfo5 < E : Endian > { pub version : U32 < E > , pub page_size : U32 < E > , pub page_starts_count : U32 < E > , reserved1 : [u8 ; 4] , pub value_add : U64 < E > , }
    };
}

DyldCacheSlideInfo5!();