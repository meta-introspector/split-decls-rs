use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A PRNG producing a 64-bit output.
///
/// The current implementation is `PCG-XSH-RR`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rand64 {
    state: u128,
    inc: u128,
}
