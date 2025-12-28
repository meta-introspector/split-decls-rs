macro_rules! deps {
    () => {
        Endian!();
    };
}

macro_rules! U64Bytes {
    () => {
        deps!();
        # [doc = " An unaligned `u64` value with an externally specified endianness of type `E`."] # [derive (Default , Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] # [repr (transparent)] pub struct U64Bytes < E : Endian > ([u8 ; 8] , PhantomData < E >) ;
    };
}

U64Bytes!()