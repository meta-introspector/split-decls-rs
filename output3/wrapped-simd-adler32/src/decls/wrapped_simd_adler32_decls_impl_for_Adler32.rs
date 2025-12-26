use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Adler32 {
    /// Constructs a new `Adler32`.
    ///
    /// Potential overhead here due to runtime feature detection although in testing on 100k
    /// and 10k random byte arrays it was not really noticeable.
    ///
    /// # Examples
    /// ```rust
    /// use simd_adler32::Adler32;
    ///
    /// let mut adler = Adler32::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }
    /// Constructs a new `Adler32` using existing checksum.
    ///
    /// Potential overhead here due to runtime feature detection although in testing on 100k
    /// and 10k random byte arrays it was not really noticeable.
    ///
    /// # Examples
    /// ```rust
    /// use simd_adler32::Adler32;
    ///
    /// let mut adler = Adler32::from_checksum(0xdeadbeaf);
    /// ```
    pub fn from_checksum(checksum: u32) -> Self {
        Self {
            a: checksum as u16,
            b: (checksum >> 16) as u16,
            update: get_imp(),
        }
    }
    /// Computes hash for supplied data and stores results in internal state.
    pub fn write(&mut self, data: &[u8]) {
        let (a, b) = (self.update)(self.a, self.b, data);
        self.a = a;
        self.b = b;
    }
    /// Returns the hash value for the values written so far.
    ///
    /// Despite its name, the method does not reset the hasher’s internal state. Additional
    /// writes will continue from the current value. If you need to start a fresh hash
    /// value, you will have to use `reset`.
    pub fn finish(&self) -> u32 {
        (u32::from(self.b) << 16) | u32::from(self.a)
    }
    /// Resets the internal state.
    pub fn reset(&mut self) {
        self.a = 1;
        self.b = 0;
    }
}
