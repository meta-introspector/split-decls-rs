macro_rules! deps {
    () => {
        DyldCacheRelocationIteratorV2!();
        Endian!();
        Endianness!();
        DyldCacheRelocationIteratorV3!();
        DyldCacheRelocationIteratorV5!();
        ReadRef!();
    };
}

macro_rules! DyldCacheRelocationIteratorVersion {
    () => {
        deps!();
        # [derive (Debug)] enum DyldCacheRelocationIteratorVersion < 'data , E = Endianness , R = & 'data [u8] > where E : Endian , R : ReadRef < 'data > , { None , V2 (DyldCacheRelocationIteratorV2 < 'data , E , R >) , V3 (DyldCacheRelocationIteratorV3 < 'data , E , R >) , V5 (DyldCacheRelocationIteratorV5 < 'data , E , R >) , }
    };
}

DyldCacheRelocationIteratorVersion!()