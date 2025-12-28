macro_rules! deps {
    () => {
        U64!();
        Endian!();
        U32!();
    };
}

macro_rules! DyldCacheMappingInfo {
    () => {
        deps!();
        # [doc = " Corresponds to struct dyld_cache_mapping_info from dyld_cache_format.h."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct DyldCacheMappingInfo < E : Endian > { pub address : U64 < E > , pub size : U64 < E > , pub file_offset : U64 < E > , pub max_prot : U32 < E > , pub init_prot : U32 < E > , }
    };
}

DyldCacheMappingInfo!()