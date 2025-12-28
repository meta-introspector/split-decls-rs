macro_rules! StartBytesTwo {
    () => {
        # [doc = " A prefilter for scanning for two starting bytes."] # [cfg (feature = "perf-literal")] # [derive (Clone , Debug)] struct StartBytesTwo { byte1 : u8 , byte2 : u8 , }
    };
}

StartBytesTwo!()