use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Trait for reader types which are used to extract extendable output
/// from a XOF (extendable-output function) result.
pub trait XofReader {
    /// Read output into the `buffer`. Can be called an unlimited number of times.
    fn read(&mut self, buffer: &mut [u8]);
    /// Read output into a boxed slice of the specified size.
    ///
    /// Can be called an unlimited number of times in combination with `read`.
    ///
    /// `Box<[u8]>` is used instead of `Vec<u8>` to save stack space, since
    /// they have size of 2 and 3 words respectively.
    #[cfg(feature = "alloc")]
    fn read_boxed(&mut self, n: usize) -> Box<[u8]> {
        let mut buf = vec![0u8; n].into_boxed_slice();
        self.read(&mut buf);
        buf
    }
}
