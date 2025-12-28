macro_rules! deps {
    () => {
        I32Bytes!();
    };
}

macro_rules! I32 {
    () => {
        deps!();
        # [doc = " An `i32` value with an externally specified endianness of type `E`."] # [cfg (feature = "unaligned")] pub type I32 < E > = I32Bytes < E > ;
    };
}

I32!();