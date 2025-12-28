macro_rules! deps {
    () => {
        ReadBuf!();
    };
}

macro_rules! ReadBufCursor {
    () => {
        deps!();
        # [doc = " The cursor part of a [`ReadBuf`]."] # [doc = ""] # [doc = " This is created by calling `ReadBuf::unfilled()`."] # [derive (Debug)] pub struct ReadBufCursor < 'a > { buf : & 'a mut ReadBuf < 'a > , }
    };
}

ReadBufCursor!();