use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "std")]
impl std::io::Write for Hasher {
    /// This is equivalent to [`update`](#method.update).
    #[inline]
    fn write(&mut self, input: &[u8]) -> std::io::Result<usize> {
        self.update(input);
        Ok(input.len())
    }
    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
