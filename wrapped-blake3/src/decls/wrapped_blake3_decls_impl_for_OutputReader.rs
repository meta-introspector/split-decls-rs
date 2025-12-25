use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl OutputReader {
    fn new(inner: Output) -> Self {
        Self {
            inner,
            position_within_block: 0,
        }
    }
    fn fill_one_block(&mut self, buf: &mut &mut [u8]) {
        let output_block: [u8; BLOCK_LEN] = self.inner.root_output_block();
        let output_bytes = &output_block[self.position_within_block as usize..];
        let take = cmp::min(buf.len(), output_bytes.len());
        buf[..take].copy_from_slice(&output_bytes[..take]);
        self.position_within_block += take as u8;
        if self.position_within_block == BLOCK_LEN as u8 {
            self.inner.counter += 1;
            self.position_within_block = 0;
        }
        *buf = &mut core::mem::take(buf)[take..];
    }
    /// Fill a buffer with output bytes and advance the position of the
    /// `OutputReader`. This is equivalent to [`Read::read`], except that it
    /// doesn't return a `Result`. Both methods always fill the entire buffer.
    ///
    /// Note that `OutputReader` doesn't buffer output bytes internally, so
    /// calling `fill` repeatedly with a short-length or odd-length slice will
    /// end up performing the same compression multiple times. If you're
    /// reading output in a loop, prefer a slice length that's a multiple of
    /// [`BLOCK_LEN`] (64 bytes).
    ///
    /// The maximum output size of BLAKE3 is 2<sup>64</sup>-1 bytes. If you try
    /// to extract more than that, for example by seeking near the end and
    /// reading further, the behavior is unspecified.
    ///
    /// [`Read::read`]: #method.read
    pub fn fill(&mut self, mut buf: &mut [u8]) {
        if buf.is_empty() {
            return;
        }
        if self.position_within_block != 0 {
            self.fill_one_block(&mut buf);
        }
        let full_blocks = buf.len() / BLOCK_LEN;
        let full_blocks_len = full_blocks * BLOCK_LEN;
        if full_blocks > 0 {
            debug_assert_eq!(0, self.position_within_block);
            self.inner.platform.xof_many(
                &self.inner.input_chaining_value,
                &self.inner.block,
                self.inner.block_len,
                self.inner.counter,
                self.inner.flags | ROOT,
                &mut buf[..full_blocks_len],
            );
            self.inner.counter += full_blocks as u64;
            buf = &mut buf[full_blocks * BLOCK_LEN..];
        }
        if !buf.is_empty() {
            debug_assert!(buf.len() < BLOCK_LEN);
            self.fill_one_block(&mut buf);
            debug_assert!(buf.is_empty());
        }
    }
    /// Return the current read position in the output stream. This is
    /// equivalent to [`Seek::stream_position`], except that it doesn't return
    /// a `Result`. The position of a new `OutputReader` starts at 0, and each
    /// call to [`fill`] or [`Read::read`] moves the position forward by the
    /// number of bytes read.
    ///
    /// [`Seek::stream_position`]: #method.stream_position
    /// [`fill`]: #method.fill
    /// [`Read::read`]: #method.read
    pub fn position(&self) -> u64 {
        self.inner.counter * BLOCK_LEN as u64 + self.position_within_block as u64
    }
    /// Seek to a new read position in the output stream. This is equivalent to
    /// calling [`Seek::seek`] with [`SeekFrom::Start`], except that it doesn't
    /// return a `Result`.
    ///
    /// [`Seek::seek`]: #method.seek
    /// [`SeekFrom::Start`]: https://doc.rust-lang.org/std/io/enum.SeekFrom.html
    pub fn set_position(&mut self, position: u64) {
        self.position_within_block = (position % BLOCK_LEN as u64) as u8;
        self.inner.counter = position / BLOCK_LEN as u64;
    }
}
