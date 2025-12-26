use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An implementation of [`BuildHasher`] that produces [`FxHasher`]s.
///
/// ```
/// use std::hash::BuildHasher;
/// use rustc_hash::FxBuildHasher;
/// assert_ne!(FxBuildHasher.hash_one(1), FxBuildHasher.hash_one(2));
/// ```
#[derive(Copy, Clone, Default)]
pub struct FxBuildHasher;
