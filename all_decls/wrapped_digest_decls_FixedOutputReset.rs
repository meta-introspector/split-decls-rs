use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Trait for hash functions with fixed-size output able to reset themselves.
pub trait FixedOutputReset: FixedOutput + Reset {
    /// Write result into provided array and reset the hasher state.
    fn finalize_into_reset(&mut self, out: &mut Output<Self>);
    /// Retrieve result and reset the hasher state.
    #[inline]
    fn finalize_fixed_reset(&mut self) -> Output<Self> {
        let mut out = Default::default();
        self.finalize_into_reset(&mut out);
        out
    }
}
