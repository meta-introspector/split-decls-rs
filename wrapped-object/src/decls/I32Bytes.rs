macro_rules! deps {
    () => {
        Endian!();
    };
}

macro_rules! I32Bytes {
    () => {
        deps!();
        # [doc = " An unaligned `i32` value with an externally specified endianness of type `E`."] # [derive (Default , Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] # [repr (transparent)] pub struct I32Bytes < E : Endian > ([u8 ; 4] , PhantomData < E >) ;
    };
}

I32Bytes!()