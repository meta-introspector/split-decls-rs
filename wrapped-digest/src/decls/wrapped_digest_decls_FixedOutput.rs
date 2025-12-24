use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Trait for hash functions with fixed-size output.
pub trait FixedOutput: Update + OutputSizeUser + Sized {
    /// Consume value and write result into provided array.
    fn finalize_into(self, out: &mut Output<Self>);
    /// Retrieve result and consume the hasher instance.
    #[inline]
    fn finalize_fixed(self) -> Output<Self> {
        let mut out = Default::default();
        self.finalize_into(&mut out);
        out
    }
}
