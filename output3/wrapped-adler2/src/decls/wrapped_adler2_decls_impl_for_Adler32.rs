use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Adler32 {
    /// Creates a new Adler-32 instance with default state.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
    /// Creates an `Adler32` instance from a precomputed Adler-32 checksum.
    ///
    /// This allows resuming checksum calculation without having to keep the `Adler32` instance
    /// around.
    ///
    /// # Example
    ///
    /// ```
    /// # use adler2::Adler32;
    /// let parts = [
    ///     "rust",
    ///     "acean",
    /// ];
    /// let whole = adler2::adler32_slice(b"rustacean");
    ///
    /// let mut sum = Adler32::new();
    /// sum.write_slice(parts[0].as_bytes());
    /// let partial = sum.checksum();
    ///
    /// // ...later
    ///
    /// let mut sum = Adler32::from_checksum(partial);
    /// sum.write_slice(parts[1].as_bytes());
    /// assert_eq!(sum.checksum(), whole);
    /// ```
    #[inline]
    pub const fn from_checksum(sum: u32) -> Self {
        Adler32 {
            a: sum as u16,
            b: (sum >> 16) as u16,
        }
    }
    /// Returns the calculated checksum at this point in time.
    #[inline]
    pub fn checksum(&self) -> u32 {
        (u32::from(self.b) << 16) | u32::from(self.a)
    }
    /// Adds `bytes` to the checksum calculation.
    ///
    /// If efficiency matters, this should be called with Byte slices that contain at least a few
    /// thousand Bytes.
    pub fn write_slice(&mut self, bytes: &[u8]) {
        self.compute(bytes);
    }
}
