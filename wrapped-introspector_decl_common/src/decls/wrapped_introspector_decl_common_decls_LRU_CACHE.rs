use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub static LRU_CACHE: Lazy<Mutex<LruCache<String, Vec<u8>>>> = Lazy::new(|| {
    let capacity = NonZeroUsize::new(1024).expect("Cache capacity must be non-zero");
    Mutex::new(LruCache::new(capacity))
});
