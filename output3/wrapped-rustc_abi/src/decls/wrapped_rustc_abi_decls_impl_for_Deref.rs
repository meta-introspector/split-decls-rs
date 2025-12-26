use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Deref for AbiAlign {
    type Target = Align;
    fn deref(&self) -> &Self::Target {
        &self.abi
    }
}
