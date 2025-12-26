use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A PRNG producing a 32-bit output.
///
/// The current implementation is `PCG-XSH-RR`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rand32 {
    state: u64,
    inc: u64,
}
