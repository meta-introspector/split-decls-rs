macro_rules! deps {
    () => {
        U64Bytes!();
    };
}

macro_rules! U64 {
    () => {
        deps!();
        # [doc = " A `u64` value with an externally specified endianness of type `E`."] # [cfg (feature = "unaligned")] pub type U64 < E > = U64Bytes < E > ;
    };
}

U64!();