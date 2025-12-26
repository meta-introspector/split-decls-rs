use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<R: gimli::Reader> Default for RangeAttributes<R> {
    fn default() -> Self {
        RangeAttributes {
            low_pc: None,
            high_pc: None,
            size: None,
            ranges_offset: None,
        }
    }
}
