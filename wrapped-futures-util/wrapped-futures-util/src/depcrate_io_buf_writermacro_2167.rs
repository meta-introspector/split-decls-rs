// Generated macro for macro_2167 (macro)
macro_rules! Depcrate_io_buf_writermacro_2167 {
() => {
// Module: crate::io::buf_writer
// Provides: {"macro_2167"}
// Dependencies: {}
pin_project ! { # [doc = " Wraps a writer and buffers its output."] # [doc = ""] # [doc = " It can be excessively inefficient to work directly with something that"] # [doc = " implements [`AsyncWrite`]. A `BufWriter` keeps an in-memory buffer of data and"] # [doc = " writes it to an underlying writer in large, infrequent batches."] # [doc = ""] # [doc = " `BufWriter` can improve the speed of programs that make *small* and"] # [doc = " *repeated* write calls to the same file or network socket. It does not"] # [doc = " help when writing very large amounts at once, or writing just one or a few"] # [doc = " times. It also provides no advantage when writing to a destination that is"] # [doc = " in memory, like a `Vec<u8>`."] # [doc = ""] # [doc = " When the `BufWriter` is dropped, the contents of its buffer will be"] # [doc = " discarded. Creating multiple instances of a `BufWriter` on the same"] # [doc = " stream can cause data loss. If you need to write out the contents of its"] # [doc = " buffer, you must manually call flush before the writer is dropped."] # [doc = ""] # [doc = " [`AsyncWrite`]: futures_io::AsyncWrite"] # [doc = " [`flush`]: super::AsyncWriteExt::flush"] # [doc = ""] pub struct BufWriter < W > { # [pin] inner : W , buf : Vec < u8 >, written : usize , } }
};
}
