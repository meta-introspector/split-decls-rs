macro_rules! deps {
    () => {
        I16Bytes!();
    };
}

macro_rules! I16 {
    () => {
        deps!();
        # [doc = " An `i16` value with an externally specified endianness of type `E`."] # [cfg (feature = "unaligned")] pub type I16 < E > = I16Bytes < E > ;
    };
}

I16!();