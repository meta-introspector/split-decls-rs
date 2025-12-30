// Generated macro for macro_266 (macro)
macro_rules! Depcrate_iomacro_266 {
() => {
// Module: crate::io
// Provides: {"macro_266"}
// Dependencies: {}
pin_project ! { # [doc = " Adds buffering to a writer."] # [doc = ""] # [doc = " It can be excessively inefficient to work directly with something that implements"] # [doc = " [`AsyncWrite`]. For example, every call to [`write()`][`AsyncWriteExt::write()`] on a TCP"] # [doc = " stream results in a system call. A [`BufWriter`] keeps an in-memory buffer of data and"] # [doc = " writes it to the underlying writer in large, infrequent batches."] # [doc = ""] # [doc = " [`BufWriter`] can improve the speed of programs that make *small* and *repeated* writes to"] # [doc = " the same file or networking socket. It does not help when writing very large amounts at"] # [doc = " once, or writing just once or a few times. It also provides no advantage when writing to a"] # [doc = " destination that is in memory, like a `Vec<u8>`."] # [doc = ""] # [doc = " Unlike [`std::io::BufWriter`], this type does not write out the contents of its buffer when"] # [doc = " it is dropped. Therefore, it is important that users explicitly flush the buffer before"] # [doc = " dropping the [`BufWriter`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::io::{AsyncWriteExt, BufWriter};"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " let mut output = Vec::new();"] # [doc = " let mut writer = BufWriter::new(&mut output);"] # [doc = ""] # [doc = " writer.write_all(b\"hello\").await?;"] # [doc = " writer.flush().await?;"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub struct BufWriter < W > { # [pin] inner : W , buf : Vec < u8 >, written : usize , } }
};
}
