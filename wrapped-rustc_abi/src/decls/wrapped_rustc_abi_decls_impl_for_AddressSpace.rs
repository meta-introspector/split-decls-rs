use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl AddressSpace {
    /// LLVM's `0` address space.
    pub const ZERO: Self = AddressSpace(0);
}
