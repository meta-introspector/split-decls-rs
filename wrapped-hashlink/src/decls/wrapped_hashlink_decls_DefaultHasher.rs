use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Default hasher, as selected by hashbrown.
#[derive(Clone)]
pub struct DefaultHasher(<hashbrown::DefaultHashBuilder as BuildHasher>::Hasher);
