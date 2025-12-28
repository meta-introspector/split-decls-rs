macro_rules! StartBytesThree {
    () => {
        # [doc = " A prefilter for scanning for three starting bytes."] # [cfg (feature = "perf-literal")] # [derive (Clone , Debug)] struct StartBytesThree { byte1 : u8 , byte2 : u8 , byte3 : u8 , }
    };
}

StartBytesThree!();