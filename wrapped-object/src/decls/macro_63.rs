macro_rules! deps {
    () => {
        U64Bytes!();
        U16Bytes!();
        I64Bytes!();
        I16Bytes!();
        I32Bytes!();
        U32Bytes!();
    };
}

macro_rules! macro_63 {
    () => {
        deps!();
        unsafe_impl_endian_pod ! (U16Bytes , U32Bytes , U64Bytes , I16Bytes , I32Bytes , I64Bytes) ;
    };
}

macro_63!()