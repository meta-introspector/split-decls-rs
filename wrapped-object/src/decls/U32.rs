macro_rules! deps {
    () => {
        U32Bytes!();
    };
}

macro_rules! U32 {
    () => {
        deps!();
        # [doc = " A `u32` value with an externally specified endianness of type `E`."] # [cfg (feature = "unaligned")] pub type U32 < E > = U32Bytes < E > ;
    };
}

U32!();