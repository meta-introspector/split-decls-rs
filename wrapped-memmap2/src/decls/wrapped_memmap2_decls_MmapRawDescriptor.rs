use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(windows)]
pub struct MmapRawDescriptor(RawHandle);
