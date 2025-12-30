// Generated macro for ReadBufCursor (struct)
macro_rules! Depcrate_rt_ioReadBufCursor {
() => {
// Module: crate::rt::io
// Provides: {"ReadBufCursor"}
// Dependencies: {}
# [doc = " The cursor part of a [`ReadBuf`]."] # [doc = ""] # [doc = " This is created by calling `ReadBuf::unfilled()`."] # [derive (Debug)] pub struct ReadBufCursor < 'a > { buf : & 'a mut ReadBuf < 'a > , }
};
}
