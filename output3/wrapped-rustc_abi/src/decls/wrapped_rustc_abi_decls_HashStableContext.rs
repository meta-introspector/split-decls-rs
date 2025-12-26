use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Requirements for a `StableHashingContext` to be used in this crate.
/// This is a hack to allow using the `HashStable_Generic` derive macro
/// instead of implementing everything in `rustc_middle`.
#[cfg(feature = "nightly")]
pub trait HashStableContext {}
