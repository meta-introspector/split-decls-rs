macro_rules! deps {
    () => {
        I64Bytes!();
    };
}

macro_rules! I64 {
    () => {
        deps!();
        # [doc = " An `i64` value with an externally specified endianness of type `E`."] # [cfg (feature = "unaligned")] pub type I64 < E > = I64Bytes < E > ;
    };
}

I64!()