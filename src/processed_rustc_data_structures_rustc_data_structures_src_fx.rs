// SRC: ../rust/compiler/rustc_data_structures/src/fx.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
use std::hash::BuildHasherDefault;

pub use crate::rustc_hash::{FxHashMap, FxHashSet, FxHasher};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=9 | LINES=18 */

pub type StdEntry<'a, K, V> = std::collections::hash_map::Entry<'a, K, V>;

pub type FxIndexMap<K, V> = indexmap::IndexMap<K, V, BuildHasherDefault<FxHasher>>;
pub type FxIndexSet<V> = indexmap::IndexSet<V, BuildHasherDefault<FxHasher>>;
pub type IndexEntry<'a, K, V> = indexmap::map::Entry<'a, K, V>;
pub type IndexOccupiedEntry<'a, K, V> = indexmap::map::OccupiedEntry<'a, K, V>;

pub use indexmap::set::MutableValues;

#[macro_export]
macro_rules! define_id_collections {
    ($map_name:ident, $set_name:ident, $entry_name:ident, $key:ty) => {
        pub type $map_name<T> = $crate::unord::UnordMap<$key, T>;
        pub type $set_name = $crate::unord::UnordSet<$key>;
        pub type $entry_name<'a, T> = $crate::fx::StdEntry<'a, $key, T>;
    };
}
/* AST_META: AST_ID=3 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=9 */

#[macro_export]
macro_rules! define_stable_id_collections {
    ($map_name:ident, $set_name:ident, $entry_name:ident, $key:ty) => {
        pub type $map_name<T> = $crate::fx::FxIndexMap<$key, T>;
        pub type $set_name = $crate::fx::FxIndexSet<$key>;
        pub type $entry_name<'a, T> = $crate::fx::IndexEntry<'a, $key, T>;
    };
}