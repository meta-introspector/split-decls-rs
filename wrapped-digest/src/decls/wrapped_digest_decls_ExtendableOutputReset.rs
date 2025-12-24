use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Trait for hash functions with extendable-output (XOF) able to reset themselves.
pub trait ExtendableOutputReset: ExtendableOutput + Reset {
    /// Retrieve XOF reader and reset hasher instance state.
    fn finalize_xof_reset(&mut self) -> Self::Reader;
    /// Finalize XOF, write result into `out`, and reset the hasher state.
    fn finalize_xof_reset_into(&mut self, out: &mut [u8]) {
        self.finalize_xof_reset().read(out);
    }
    /// Retrieve result into a boxed slice of the specified size and reset
    /// the hasher state.
    ///
    /// `Box<[u8]>` is used instead of `Vec<u8>` to save stack space, since
    /// they have size of 2 and 3 words respectively.
    #[cfg(feature = "alloc")]
    fn finalize_boxed_reset(&mut self, output_size: usize) -> Box<[u8]> {
        let mut buf = vec![0u8; output_size].into_boxed_slice();
        self.finalize_xof_reset().read(&mut buf);
        buf
    }
}
