use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Figure title
pub struct Title(Cow<'static, str>);
