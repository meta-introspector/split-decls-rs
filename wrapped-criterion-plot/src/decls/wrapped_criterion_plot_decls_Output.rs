use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Output file path
pub struct Output(Cow<'static, Path>);
