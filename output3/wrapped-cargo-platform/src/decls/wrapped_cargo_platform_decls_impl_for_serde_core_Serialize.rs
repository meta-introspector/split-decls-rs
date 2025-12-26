use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl serde_core::Serialize for Platform {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        self.to_string().serialize(s)
    }
}
