macro_rules! deps {
    () => {
        RareByteOffset!();
    };
}

macro_rules! RareBytesOne {
    () => {
        deps!();
        # [doc = " A prefilter for scanning for a single \"rare\" byte."] # [cfg (feature = "perf-literal")] # [derive (Clone , Debug)] struct RareBytesOne { byte1 : u8 , offset : RareByteOffset , }
    };
}

RareBytesOne!()