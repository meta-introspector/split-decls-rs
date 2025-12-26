use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl ops::Deref for JodChild {
    type Target = std::process::Child;
    fn deref(&self) -> &std::process::Child {
        &self.0
    }
}
