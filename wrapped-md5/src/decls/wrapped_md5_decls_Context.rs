use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A context.
#[derive(Clone)]
pub struct Context {
    buffer: [u8; 64],
    count: u64,
    state: [u32; 4],
}
