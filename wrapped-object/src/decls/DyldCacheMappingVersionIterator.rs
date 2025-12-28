macro_rules! deps {
    () => {
        Endian!();
        Endianness!();
        DyldCacheMappingAndSlideInfo!();
        DyldCacheMappingInfo!();
    };
}

macro_rules! DyldCacheMappingVersionIterator {
    () => {
        deps!();
        # [derive (Debug)] enum DyldCacheMappingVersionIterator < 'data , E = Endianness > where E : Endian , { V1 (slice :: Iter < 'data , macho :: DyldCacheMappingInfo < E > >) , V2 (slice :: Iter < 'data , macho :: DyldCacheMappingAndSlideInfo < E > >) , }
    };
}

DyldCacheMappingVersionIterator!();