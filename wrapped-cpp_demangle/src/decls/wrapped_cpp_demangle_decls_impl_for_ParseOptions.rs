use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl ParseOptions {
    /// Set the limit on recursion depth during the parsing phase. A low
    /// limit will cause valid symbols to be rejected, but a high limit may
    /// allow pathological symbols to overflow the stack during parsing.
    /// The default value is 96, which will not overflow the stack even in
    /// a debug build.
    pub fn recursion_limit(mut self, limit: u32) -> Self {
        self.recursion_limit = Some(
            NonZeroU32::new(limit).expect("Recursion limit must be > 0"),
        );
        self
    }
}
