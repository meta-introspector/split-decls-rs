use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[doc(hidden)]
pub struct ArgPrinter<'a, T>(pub &'a T);
