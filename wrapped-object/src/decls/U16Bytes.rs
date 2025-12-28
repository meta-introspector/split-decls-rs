macro_rules! deps {
    () => {
        Endian!();
    };
}

macro_rules! U16Bytes {
    () => {
        deps!();
        # [doc = " An unaligned `u16` value with an externally specified endianness of type `E`."] # [derive (Default , Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] # [repr (transparent)] pub struct U16Bytes < E : Endian > ([u8 ; 2] , PhantomData < E >) ;
    };
}

U16Bytes!()