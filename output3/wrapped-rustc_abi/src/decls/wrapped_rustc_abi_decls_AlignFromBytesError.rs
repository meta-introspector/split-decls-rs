use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Clone, Copy)]
pub enum AlignFromBytesError {
    NotPowerOfTwo(u64),
    TooLarge(u64),
}
