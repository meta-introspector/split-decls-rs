use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An identifier that specifies the address space that some operation
/// should operate on. Special address spaces have an effect on code generation,
/// depending on the target and the address spaces it implements.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "nightly", derive(HashStable_Generic))]
pub struct AddressSpace(pub u32);
