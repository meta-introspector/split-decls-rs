use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, Copy)]
pub(crate) struct LiteralSubCoder {
    probs: [u16; 0x300],
}
