macro_rules! deps {
    () => {
        U64!();
        U32!();
        Endian!();
    };
}

macro_rules! DyldCacheMappingAndSlideInfo {
    () => {
        deps!();
        # [doc = " Corresponds to struct dyld_cache_mapping_and_slide_info from dyld_cache_format.h."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct DyldCacheMappingAndSlideInfo < E : Endian > { pub address : U64 < E > , pub size : U64 < E > , pub file_offset : U64 < E > , pub slide_info_file_offset : U64 < E > , pub slide_info_file_size : U64 < E > , pub flags : U64 < E > , pub max_prot : U32 < E > , pub init_prot : U32 < E > , }
    };
}

DyldCacheMappingAndSlideInfo!();