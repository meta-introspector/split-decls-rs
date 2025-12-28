macro_rules! deps {
    () => {
        BlockBuffer!();
        Sealed!();
    };
}

macro_rules! SerializedBufferSize {
    () => {
        deps!();
        # [doc = " Size of serialized `BlockBuffer` in bytes."] pub type SerializedBufferSize < BS , K > = Sum < BS , < K as sealed :: Sealed > :: Overhead > ;
    };
}

SerializedBufferSize!()