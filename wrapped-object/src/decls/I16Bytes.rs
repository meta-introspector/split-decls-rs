macro_rules! deps {
    () => {
        Endian!();
    };
}

macro_rules! I16Bytes {
    () => {
        deps!();
        # [doc = " An unaligned `i16` value with an externally specified endianness of type `E`."] # [derive (Default , Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] # [repr (transparent)] pub struct I16Bytes < E : Endian > ([u8 ; 2] , PhantomData < E >) ;
    };
}

I16Bytes!()