macro_rules! deps {
    () => {
        ArrayEncoding!();
    };
}

macro_rules! ByteArray {
    () => {
        deps!();
        # [doc = " Alias for a byte array whose size is defined by [`ArrayEncoding::ByteSize`]."] pub type ByteArray < T > = Array < u8 , < T as ArrayEncoding > :: ByteSize > ;
    };
}

ByteArray!()