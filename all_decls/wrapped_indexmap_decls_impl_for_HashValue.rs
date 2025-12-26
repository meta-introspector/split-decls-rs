use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl HashValue {
    #[inline(always)]
    fn get(self) -> u64 {
        self.0 as u64
    }
}
