use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Default hash builder, matches hashbrown's default hasher.
///
/// See [`DefaultHasher`] for more details.
#[derive(Clone, Default, Debug)]
pub struct DefaultHashBuilder(hashbrown::DefaultHashBuilder);
