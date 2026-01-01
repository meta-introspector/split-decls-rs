// SRC: ../rust/compiler/rustc_type_ir/src/data_structures/mod.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
use std::hash::BuildHasherDefault;

pub use ena::unify::{NoError, UnifyKey, UnifyValue};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use crate::rustc_hash::FxHasher;
pub use crate::rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=3 | LINES=11 */

pub type IndexMap<K, V> = indexmap::IndexMap<K, V, BuildHasherDefault<FxHasher>>;
pub type IndexSet<V> = indexmap::IndexSet<V, BuildHasherDefault<FxHasher>>;


#[cfg(feature = "nightly")]
mod impl_ {
    pub use crate::rustc_data_structures::sso::{SsoHashMap, SsoHashSet};
    pub use crate::rustc_data_structures::stack::ensure_sufficient_stack;
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=ensure_sufficient_stack | COMPLEXITY=4 | LINES=10 */

#[cfg(not(feature = "nightly"))]
mod impl_ {
    pub use std::collections::{HashMap as SsoHashMap, HashSet as SsoHashSet};

    #[inline]
    pub fn ensure_sufficient_stack<R>(f: impl FnOnce() -> R) -> R {
        f()
    }
}
/* AST_META: AST_ID=5 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */

pub use delayed_map::{DelayedMap, DelayedSet};
/* AST_META: AST_ID=6 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=1 | LINES=1 */
pub use impl_::*;