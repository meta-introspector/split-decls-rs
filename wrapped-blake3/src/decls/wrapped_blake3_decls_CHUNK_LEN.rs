use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The number of bytes in a chunk, 1024.
///
/// You don't usually need to think about this number, but it often comes up in benchmarks, because
/// the maximum degree of parallelism used by the implementation equals the number of chunks.
pub const CHUNK_LEN: usize = 1024;
