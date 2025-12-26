use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub fn opaque<T: Debug>(value: &T) -> Opaque {
    Opaque(format!("{value:?}"))
}
