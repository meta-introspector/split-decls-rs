use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl_from!(TypeParam, ConstParam, LifetimeParam for GenericParam);
