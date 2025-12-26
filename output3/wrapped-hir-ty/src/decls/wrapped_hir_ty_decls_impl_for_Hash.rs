use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Hash for FnAbi {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(&Self::Unknown).hash(state);
    }
}
