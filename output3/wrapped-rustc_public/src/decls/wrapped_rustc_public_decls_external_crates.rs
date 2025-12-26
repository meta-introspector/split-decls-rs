use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Try to find a crate with the given name.
pub fn external_crates() -> Vec<Crate> {
    with(|cx| cx.external_crates())
}
