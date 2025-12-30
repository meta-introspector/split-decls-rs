// Generated macro for copy (function)
macro_rules! Depcrate_io_copycopy {
() => {
// Module: crate::io::copy
// Provides: {"copy"}
// Dependencies: {}
# [doc = " Creates a future which copies all the bytes from one object to another."] # [doc = ""] # [doc = " The returned future will copy all the bytes read from this `AsyncRead` into the"] # [doc = " `writer` specified. This future will only complete once the `reader` has hit"] # [doc = " EOF and all bytes have been written to and flushed from the `writer`"] # [doc = " provided."] # [doc = ""] # [doc = " On success the number of bytes is returned."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use futures::io::{self, AsyncWriteExt, Cursor};"] # [doc = ""] # [doc = " let reader = Cursor::new([1, 2, 3, 4]);"] # [doc = " let mut writer = Cursor::new(vec![0u8; 5]);"] # [doc = ""] # [doc = " let bytes = io::copy(reader, &mut writer).await?;"] # [doc = " writer.close().await?;"] # [doc = ""] # [doc = " assert_eq!(bytes, 4);"] # [doc = " assert_eq!(writer.into_inner(), [1, 2, 3, 4, 0]);"] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(()) }).unwrap();"] # [doc = " ```"] pub fn copy < R , W > (reader : R , writer : & mut W) -> Copy < '_ , R , W > where R : AsyncRead , W : AsyncWrite + Unpin + ? Sized , { Copy { inner : copy_buf (BufReader :: new (reader) , writer) } }
};
}
