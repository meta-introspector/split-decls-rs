use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A `Symbol` which owns the underlying storage for the mangled name.
pub type OwnedSymbol = Symbol<Vec<u8>>;
