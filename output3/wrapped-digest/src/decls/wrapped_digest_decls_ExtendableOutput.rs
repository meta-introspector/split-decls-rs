use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Trait for hash functions with extendable-output (XOF).
pub trait ExtendableOutput: Sized + Update {
    /// Reader
    type Reader: XofReader;
    /// Retrieve XOF reader and consume hasher instance.
    fn finalize_xof(self) -> Self::Reader;
    /// Finalize XOF and write result into `out`.
    fn finalize_xof_into(self, out: &mut [u8]) {
        self.finalize_xof().read(out);
    }
    /// Compute hash of `data` and write it into `output`.
    fn digest_xof(input: impl AsRef<[u8]>, output: &mut [u8])
    where
        Self: Default,
    {
        let mut hasher = Self::default();
        hasher.update(input.as_ref());
        hasher.finalize_xof().read(output);
    }
    /// Retrieve result into a boxed slice of the specified size and consume
    /// the hasher.
    ///
    /// `Box<[u8]>` is used instead of `Vec<u8>` to save stack space, since
    /// they have size of 2 and 3 words respectively.
    #[cfg(feature = "alloc")]
    fn finalize_boxed(self, output_size: usize) -> Box<[u8]> {
        let mut buf = vec![0u8; output_size].into_boxed_slice();
        self.finalize_xof().read(&mut buf);
        buf
    }
}
