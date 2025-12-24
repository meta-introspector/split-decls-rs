use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T: ArraySize + sealed::BlockSizes> BlockSizes for T {}
