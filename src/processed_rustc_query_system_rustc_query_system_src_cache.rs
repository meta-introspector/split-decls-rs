// SRC: ../rust/compiler/rustc_query_system/src/cache.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=4 | LINES=8 */
// Cache for candidate selection.

use std::hash::Hash;

use crate::rustc_data_structures::fx::FxHashMap;
use crate::rustc_data_structures::sync::Lock;

use crate::dep_graph::{DepContext, DepNodeIndex};
/* AST_META: AST_ID=2 | TYPE=STRUCT | NAME=Cache | COMPLEXITY=2 | LINES=4 */

pub struct Cache<Key, Value> {
    hashmap: Lock<FxHashMap<Key, WithDepNode<Value>>>,
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=clone | COMPLEXITY=6 | LINES=6 */

impl<Key: Clone, Value: Clone> Clone for Cache<Key, Value> {
    fn clone(&self) -> Self {
        Self { hashmap: Lock::new(self.hashmap.borrow().clone()) }
    }
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=default | COMPLEXITY=6 | LINES=6 */

impl<Key, Value> Default for Cache<Key, Value> {
    fn default() -> Self {
        Self { hashmap: Default::default() }
    }
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=clear | COMPLEXITY=3 | LINES=7 */

impl<Key, Value> Cache<Key, Value> {
    /// Actually frees the underlying memory in contrast to what stdlib containers do on `clear`
    pub fn clear(&self) {
        *self.hashmap.borrow_mut() = Default::default();
    }
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=get | COMPLEXITY=4 | LINES=10 */

impl<Key: Eq + Hash, Value: Clone> Cache<Key, Value> {
    pub fn get<Tcx: DepContext>(&self, key: &Key, tcx: Tcx) -> Option<Value> {
        Some(self.hashmap.borrow().get(key)?.get(tcx))
    }

    pub fn insert(&self, key: Key, dep_node: DepNodeIndex, value: Value) {
        self.hashmap.borrow_mut().insert(key, WithDepNode::new(dep_node, value));
    }
}
/* AST_META: AST_ID=7 | TYPE=STRUCT | NAME=WithDepNode | COMPLEXITY=2 | LINES=6 */

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WithDepNode<T> {
    dep_node: DepNodeIndex,
    cached_value: T,
}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=new | COMPLEXITY=5 | LINES=11 */

impl<T: Clone> WithDepNode<T> {
    pub fn new(dep_node: DepNodeIndex, cached_value: T) -> Self {
        WithDepNode { dep_node, cached_value }
    }

    pub fn get<Tcx: DepContext>(&self, tcx: Tcx) -> T {
        tcx.dep_graph().read_index(self.dep_node);
        self.cached_value.clone()
    }
}