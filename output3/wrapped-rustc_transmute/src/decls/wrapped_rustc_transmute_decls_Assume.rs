use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Copy, Clone, Debug, Default)]
pub struct Assume {
    pub alignment: bool,
    pub lifetimes: bool,
    pub safety: bool,
    pub validity: bool,
}
