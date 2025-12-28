macro_rules! deps {
    () => {
        DyldCacheSlideInfo2!();
        U16!();
        DyldCacheSlideInfo5!();
        DyldCacheSlideInfo3!();
        Endian!();
    };
}

macro_rules! DyldCacheSlideInfo {
    () => {
        deps!();
        # [doc = " The slide info for a dyld cache mapping, including variable length arrays."] # [derive (Debug , Clone , Copy)] # [non_exhaustive] # [allow (missing_docs)] pub enum DyldCacheSlideInfo < 'data , E : Endian > { None , V2 { slide : & 'data macho :: DyldCacheSlideInfo2 < E > , page_starts : & 'data [U16 < E >] , page_extras : & 'data [U16 < E >] , } , V3 { slide : & 'data macho :: DyldCacheSlideInfo3 < E > , page_starts : & 'data [U16 < E >] , } , V5 { slide : & 'data macho :: DyldCacheSlideInfo5 < E > , page_starts : & 'data [U16 < E >] , } , }
    };
}

DyldCacheSlideInfo!();