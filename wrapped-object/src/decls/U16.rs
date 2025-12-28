macro_rules! deps {
    () => {
        U16Bytes!();
    };
}

macro_rules! U16 {
    () => {
        deps!();
        # [doc = " A `u16` value with an externally specified endianness of type `E`."] # [cfg (feature = "unaligned")] pub type U16 < E > = U16Bytes < E > ;
    };
}

U16!()