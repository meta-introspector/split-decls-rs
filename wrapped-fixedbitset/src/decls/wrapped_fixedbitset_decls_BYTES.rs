use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "serde")]
pub(crate) const BYTES: usize = core::mem::size_of::<Block>();
