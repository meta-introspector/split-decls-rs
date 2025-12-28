macro_rules! deps {
    () => {
        RareByteOffsets!();
    };
}

macro_rules! RareBytesThree {
    () => {
        deps!();
        # [doc = " A prefilter for scanning for three \"rare\" bytes."] # [cfg (feature = "perf-literal")] # [derive (Clone , Debug)] struct RareBytesThree { offsets : RareByteOffsets , byte1 : u8 , byte2 : u8 , byte3 : u8 , }
    };
}

RareBytesThree!();