macro_rules! deps {
    () => {
        BufferKind!();
    };
}

macro_rules! BlockBuffer {
    () => {
        deps!();
        # [doc = " Buffer for block processing of data."] pub struct BlockBuffer < BS : ArraySize , K : BufferKind > { buffer : MaybeUninit < Array < u8 , BS > > , pos : K :: Pos , }
    };
}

BlockBuffer!();