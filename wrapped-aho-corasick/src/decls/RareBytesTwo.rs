macro_rules! deps {
    () => {
        RareByteOffsets!();
    };
}

macro_rules! RareBytesTwo {
    () => {
        deps!();
        # [doc = " A prefilter for scanning for two \"rare\" bytes."] # [cfg (feature = "perf-literal")] # [derive (Clone , Debug)] struct RareBytesTwo { offsets : RareByteOffsets , byte1 : u8 , byte2 : u8 , }
    };
}

RareBytesTwo!()