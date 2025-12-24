use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Hash for Ident {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.to_string().hash(hasher);
    }
}
