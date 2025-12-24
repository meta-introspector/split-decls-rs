use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl StaticLifetime {
    pub fn name(self) -> Name {
        Name::new_symbol_root(sym::tick_static)
    }
}
