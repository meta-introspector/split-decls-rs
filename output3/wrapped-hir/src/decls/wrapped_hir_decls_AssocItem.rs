use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Invariant: `inner.as_assoc_item(db).is_some()`
/// We do not actively enforce this invariant.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AssocItem {
    Function(Function),
    Const(Const),
    TypeAlias(TypeAlias),
}
