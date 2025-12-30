// Generated macro for Cursor (struct)
macro_rules! Depcrate_io_cursorCursor {
() => {
// Module: crate::io::cursor
// Provides: {"Cursor"}
// Dependencies: {}
# [doc = " A `Cursor` wraps an in-memory buffer and provides it with a"] # [doc = " [`AsyncSeek`] implementation."] # [doc = ""] # [doc = " `Cursor`s are used with in-memory buffers, anything implementing"] # [doc = " `AsRef<[u8]>`, to allow them to implement [`AsyncRead`] and/or [`AsyncWrite`],"] # [doc = " allowing these buffers to be used anywhere you might use a reader or writer"] # [doc = " that does actual I/O."] # [doc = ""] # [doc = " This library implements some I/O traits on various types which"] # [doc = " are commonly used as a buffer, like `Cursor<`[`Vec`]`<u8>>` and"] # [doc = " `Cursor<`[`&[u8]`][bytes]`>`."] # [doc = ""] # [doc = " [`AsyncSeek`]: trait.AsyncSeek.html"] # [doc = " [`AsyncRead`]: trait.AsyncRead.html"] # [doc = " [`AsyncWrite`]: trait.AsyncWrite.html"] # [doc = " [bytes]: https://doc.rust-lang.org/std/primitive.slice.html"] # [derive (Clone , Debug , Default)] pub struct Cursor < T > { inner : io :: Cursor < T > , }
};
}
