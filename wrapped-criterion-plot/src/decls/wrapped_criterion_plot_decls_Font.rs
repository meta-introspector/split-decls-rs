use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A font name
pub struct Font(Cow<'static, str>);
