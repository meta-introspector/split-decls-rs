use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, Copy)]
pub struct MiniCore<'a>(&'a str);
