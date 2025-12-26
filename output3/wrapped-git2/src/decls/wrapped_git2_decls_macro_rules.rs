use serde::{Deserialize, Serialize};
use std::collections::HashMap;
macro_rules! is_bit_set {
    ($name:ident, $flag:expr) => {
        #[allow(missing_docs)]
        pub fn $name(&self) -> bool {
            self.intersects($flag)
        }
    };
}
