use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Thread-local variable wrapper
///
/// See the [module-level documentation](index.html) for more.
pub struct ThreadLocal<T: Send> {
    /// The buckets in the thread local. The nth bucket contains `2^n`
    /// elements. Each bucket is lazily allocated.
    buckets: [AtomicPtr<Entry<T>>; BUCKETS],
    /// The number of values in the thread local. This can be less than the real number of values,
    /// but is never more.
    values: AtomicUsize,
}
