use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A 128-bit (16 byte) buffer containing the UUID.
///
/// # ABI
///
/// The `Bytes` type is always guaranteed to be have the same ABI as [`Uuid`].
pub type Bytes = [u8; 16];
