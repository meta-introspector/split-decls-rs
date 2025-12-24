use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl HasCrate for Module {
    fn krate(&self, _: &dyn HirDatabase) -> Crate {
        Module::krate(*self)
    }
}
