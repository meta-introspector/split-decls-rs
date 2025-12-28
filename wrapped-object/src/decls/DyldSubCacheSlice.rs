macro_rules! deps {
    () => {
        Endian!();
        DyldSubCacheEntryV1!();
        DyldSubCacheEntryV2!();
    };
}

macro_rules! DyldSubCacheSlice {
    () => {
        deps!();
        # [doc = " A slice of structs describing each subcache."] # [doc = ""] # [doc = " The struct gained an additional field (the file suffix) in dyld-1042.1 (macOS 13 / iOS 16),"] # [doc = " so this is an enum of the two possible slice types."] # [derive (Debug , Clone , Copy)] # [non_exhaustive] pub enum DyldSubCacheSlice < 'data , E : Endian > { # [doc = " V1, used between dyld-940 and dyld-1042.1."] V1 (& 'data [macho :: DyldSubCacheEntryV1 < E >]) , # [doc = " V2, used since dyld-1042.1."] V2 (& 'data [macho :: DyldSubCacheEntryV2 < E >]) , }
    };
}

DyldSubCacheSlice!()