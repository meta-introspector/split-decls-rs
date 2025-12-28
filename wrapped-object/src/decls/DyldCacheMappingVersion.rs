macro_rules! deps {
    () => {
        DyldCacheMappingAndSlideInfo!();
        DyldCacheMappingInfo!();
        Endian!();
        Endianness!();
    };
}

macro_rules! DyldCacheMappingVersion {
    () => {
        deps!();
        # [derive (Clone , Copy)] enum DyldCacheMappingVersion < 'data , E = Endianness > where E : Endian , { V1 (& 'data macho :: DyldCacheMappingInfo < E >) , V2 (& 'data macho :: DyldCacheMappingAndSlideInfo < E >) , }
    };
}

DyldCacheMappingVersion!();