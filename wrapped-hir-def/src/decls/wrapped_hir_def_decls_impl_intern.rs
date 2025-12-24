use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl_intern!(BlockId, BlockLoc, intern_block, lookup_intern_block);
