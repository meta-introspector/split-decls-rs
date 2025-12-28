macro_rules! deps {
    () => {
        BlockBuffer!();
        SerializedBufferSize!();
    };
}

macro_rules! SerializedBuffer {
    () => {
        deps!();
        # [doc = " `BlockBuffer` serialized as a byte array."] pub type SerializedBuffer < BS , K > = Array < u8 , SerializedBufferSize < BS , K > > ;
    };
}

SerializedBuffer!()