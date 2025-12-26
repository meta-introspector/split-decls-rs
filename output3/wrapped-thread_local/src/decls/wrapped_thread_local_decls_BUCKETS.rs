use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The total number of buckets stored in each thread local.
/// All buckets combined can hold up to `usize::MAX - 1` entries.
const BUCKETS: usize = (usize::BITS - 1) as usize;
