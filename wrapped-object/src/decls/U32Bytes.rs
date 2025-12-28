macro_rules! deps {
    () => {
        Endian!();
    };
}

macro_rules! U32Bytes {
    () => {
        deps!();
        # [doc = " An unaligned `u32` value with an externally specified endianness of type `E`."] # [derive (Default , Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] # [repr (transparent)] pub struct U32Bytes < E : Endian > ([u8 ; 4] , PhantomData < E >) ;
    };
}

U32Bytes!()