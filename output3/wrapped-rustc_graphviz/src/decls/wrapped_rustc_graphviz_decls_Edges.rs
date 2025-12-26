use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub type Edges<'a, E> = Cow<'a, [E]>;
