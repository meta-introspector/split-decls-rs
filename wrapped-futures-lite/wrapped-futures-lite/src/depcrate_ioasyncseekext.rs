// Generated macro for AsyncSeekExt (trait)
macro_rules! Depcrate_ioAsyncSeekExt {
() => {
// Module: crate::io
// Provides: {"AsyncSeekExt"}
// Dependencies: {}
# [doc = " Extension trait for [`AsyncSeek`]."] pub trait AsyncSeekExt : AsyncSeek { # [doc = " Seeks to a new position in a byte stream."] # [doc = ""] # [doc = " Returns the new position in the byte stream."] # [doc = ""] # [doc = " A seek beyond the end of stream is allowed, but behavior is defined by the implementation."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::io::{AsyncSeekExt, Cursor, SeekFrom};"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " let mut cursor = Cursor::new(\"hello\");"] # [doc = ""] # [doc = " // Move the cursor to the end."] # [doc = " cursor.seek(SeekFrom::End(0)).await?;"] # [doc = ""] # [doc = " // Check the current position."] # [doc = " assert_eq!(cursor.seek(SeekFrom::Current(0)).await?, 5);"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] fn seek (& mut self , pos : SeekFrom) -> SeekFuture < '_ , Self > where Self : Unpin , { SeekFuture { seeker : self , pos } } }
};
}
