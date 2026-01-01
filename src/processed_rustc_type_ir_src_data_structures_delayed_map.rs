// SRC: ../rust/compiler/rustc_type_ir/src/data_structures/delayed_map.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
use std::hash::Hash;

use crate::data_structures::{HashMap, HashSet};
/* AST_META: AST_ID=2 | TYPE=STRUCT | NAME=DelayedMap | COMPLEXITY=4 | LINES=13 */

const CACHE_CUTOFF: u32 = 32;

/// A hashmap which only starts hashing after ignoring the first few inputs.
///
/// This is used in type folders as in nearly all cases caching is not worth it
/// as nearly all folded types are tiny. However, there are very rare incredibly
/// large types for which caching is necessary to avoid hangs.
#[derive(Debug)]
pub struct DelayedMap<K, V> {
    cache: HashMap<K, V>,
    count: u32,
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=default | COMPLEXITY=6 | LINES=6 */

impl<K, V> Default for DelayedMap<K, V> {
    fn default() -> Self {
        DelayedMap { cache: Default::default(), count: 0 }
    }
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=insert | COMPLEXITY=15 | LINES=29 */

impl<K: Hash + Eq, V> DelayedMap<K, V> {
    #[inline(always)]
    pub fn insert(&mut self, key: K, value: V) -> bool {
        if self.count >= CACHE_CUTOFF {
            self.cold_insert(key, value)
        } else {
            self.count += 1;
            true
        }
    }

    #[cold]
    #[inline(never)]
    fn cold_insert(&mut self, key: K, value: V) -> bool {
        self.cache.insert(key, value).is_none()
    }

    #[inline(always)]
    pub fn get(&self, key: &K) -> Option<&V> {
        if self.cache.is_empty() { None } else { self.cold_get(key) }
    }

    #[cold]
    #[inline(never)]
    fn cold_get(&self, key: &K) -> Option<&V> {
        self.cache.get(key)
    }
}
/* AST_META: AST_ID=5 | TYPE=STRUCT | NAME=DelayedSet | COMPLEXITY=2 | LINES=6 */

#[derive(Debug)]
pub struct DelayedSet<T> {
    cache: HashSet<T>,
    count: u32,
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=default | COMPLEXITY=6 | LINES=6 */

impl<T> Default for DelayedSet<T> {
    fn default() -> Self {
        DelayedSet { cache: Default::default(), count: 0 }
    }
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=insert | COMPLEXITY=11 | LINES=29 */

impl<T: Hash + Eq> DelayedSet<T> {
    #[inline(always)]
    pub fn insert(&mut self, value: T) -> bool {
        if self.count >= CACHE_CUTOFF {
            self.cold_insert(value)
        } else {
            self.count += 1;
            true
        }
    }

    #[cold]
    #[inline(never)]
    fn cold_insert(&mut self, value: T) -> bool {
        self.cache.insert(value)
    }

    #[inline(always)]
    pub fn contains(&self, value: &T) -> bool {
        !self.cache.is_empty() && self.cold_contains(value)
    }

    #[cold]
    #[inline(never)]
    fn cold_contains(&self, value: &T) -> bool {
        self.cache.contains(value)
    }
}