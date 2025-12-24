use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Position within [`RawArgs`]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArgCursor {
    cursor: usize,
}
