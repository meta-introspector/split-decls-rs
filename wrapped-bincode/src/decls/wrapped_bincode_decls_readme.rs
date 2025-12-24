use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(all(feature = "alloc", feature = "derive", doctest))]
mod readme {
    #![doc = include_str!("../readme.md")]
}
