// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_query_system/src/cache.rs
// Error: expected square brackets
// Problematic line: line 10


use crate::dep_graph::{DepContext, DepNodeIndex};

pub struct Cache<Key, Value> {
    hashmap: Lock<FxHashMap<Key, WithDepNode<Value>>>,
}

