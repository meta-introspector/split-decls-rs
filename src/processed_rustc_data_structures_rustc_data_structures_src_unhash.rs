// SRC: ../rust/compiler/rustc_data_structures/src/unhash.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use std::collections::{HashMap, HashSet};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use std::hash::{BuildHasherDefault, Hasher};
/* AST_META: AST_ID=3 | TYPE=STRUCT | NAME=Unhasher | COMPLEXITY=2 | LINES=11 */

pub type UnhashMap<K, V> = HashMap<K, V, BuildHasherDefault<Unhasher>>;
pub type UnhashSet<V> = HashSet<V, BuildHasherDefault<Unhasher>>;
pub type UnindexMap<K, V> = indexmap::IndexMap<K, V, BuildHasherDefault<Unhasher>>;

/// This no-op hasher expects only a single `write_u64` call. It's intended for
/// map keys that already have hash-like quality, like `Fingerprint`.
#[derive(Default)]
pub struct Unhasher {
    value: u64,
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=finish | COMPLEXITY=7 | LINES=17 */

impl Hasher for Unhasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.value
    }

    fn write(&mut self, _bytes: &[u8]) {
        unimplemented!("use write_u64");
    }

    #[inline]
    fn write_u64(&mut self, value: u64) {
        debug_assert_eq!(0, self.value, "Unhasher doesn't mix values!");
        self.value = value;
    }
}