use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "encoder")]
struct CountingWriter<W> {
    inner: W,
    bytes_written: u64,
}
