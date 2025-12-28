macro_rules! deps {
    () => {
        Endian!();
        U32!();
        U64!();
    };
}

macro_rules! DyldCacheImageInfo {
    () => {
        deps!();
        # [doc = " Corresponds to struct dyld_cache_image_info from dyld_cache_format.h."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct DyldCacheImageInfo < E : Endian > { pub address : U64 < E > , pub mod_time : U64 < E > , pub inode : U64 < E > , pub path_file_offset : U32 < E > , pub pad : U32 < E > , }
    };
}

DyldCacheImageInfo!();