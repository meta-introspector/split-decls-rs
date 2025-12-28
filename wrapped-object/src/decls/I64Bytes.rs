macro_rules! deps {
    () => {
        Endian!();
    };
}

macro_rules! I64Bytes {
    () => {
        deps!();
        # [doc = " An unaligned `i64` value with an externally specified endianness of type `E`."] # [derive (Default , Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] # [repr (transparent)] pub struct I64Bytes < E : Endian > ([u8 ; 8] , PhantomData < E >) ;
    };
}

I64Bytes!()