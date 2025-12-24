use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Plot label
pub struct Label(Cow<'static, str>);
