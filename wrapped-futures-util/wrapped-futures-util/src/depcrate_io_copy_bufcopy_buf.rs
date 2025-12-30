// Generated macro for copy_buf (function)
macro_rules! Depcrate_io_copy_bufcopy_buf {
() => {
// Module: crate::io::copy_buf
// Provides: {"copy_buf"}
// Dependencies: {}
# [doc = " Creates a future which copies all the bytes from one object to another."] # [doc = ""] # [doc = " The returned future will copy all the bytes read from this `AsyncBufRead` into the"] # [doc = " `writer` specified. This future will only complete once the `reader` has hit"] # [doc = " EOF and all bytes have been written to and flushed from the `writer`"] # [doc = " provided."] # [doc = ""] # [doc = " On success the number of bytes is returned."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use futures::io::{self, AsyncWriteExt, Cursor};"] # [doc = ""] # [doc = " let reader = Cursor::new([1, 2, 3, 4]);"] # [doc = " let mut writer = Cursor::new(vec![0u8; 5]);"] # [doc = ""] # [doc = " let bytes = io::copy_buf(reader, &mut writer).await?;"] # [doc = " writer.close().await?;"] # [doc = ""] # [doc = " assert_eq!(bytes, 4);"] # [doc = " assert_eq!(writer.into_inner(), [1, 2, 3, 4, 0]);"] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(()) }).unwrap();"] # [doc = " ```"] pub fn copy_buf < R , W > (reader : R , writer : & mut W) -> CopyBuf < '_ , R , W > where R : AsyncBufRead , W : AsyncWrite + Unpin + ? Sized , { CopyBuf { reader , writer , amt : 0 } }
};
}
