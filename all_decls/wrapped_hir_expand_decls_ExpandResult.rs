use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub type ExpandResult<T> = ValueResult<T, ExpandError>;
