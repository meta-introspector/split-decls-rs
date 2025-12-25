use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Kind of write access to a value
/// (For informational purposes only)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum WriteKind {
    StorageDeadOrDrop,
    Replace,
    MutableBorrow(BorrowKind),
    Mutate,
    Move,
}
