use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(feature = "rustc-dep-of-std"))]
impl StreamResult {
    #[inline]
    pub const fn error(error: MZError) -> StreamResult {
        StreamResult {
            bytes_consumed: 0,
            bytes_written: 0,
            status: Err(error),
        }
    }
}
