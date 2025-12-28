macro_rules! StartBytesOne {
    () => {
        # [doc = " A prefilter for scanning for a single starting byte."] # [cfg (feature = "perf-literal")] # [derive (Clone , Debug)] struct StartBytesOne { byte1 : u8 , }
    };
}

StartBytesOne!()