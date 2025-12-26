use serde::{Deserialize, Serialize};
use std::collections::HashMap;
struct CountingReader<R> {
    inner: R,
    bytes_read: u64,
}
