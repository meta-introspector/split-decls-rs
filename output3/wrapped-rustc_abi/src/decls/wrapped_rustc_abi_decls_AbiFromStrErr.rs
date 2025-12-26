use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Clone, Debug)]
pub enum AbiFromStrErr {
    /// not a known ABI
    Unknown,
    /// no "-unwind" variant can be used here
    NoExplicitUnwind,
}
