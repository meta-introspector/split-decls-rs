use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DebugFile {
    Primary,
    Supplementary,
    Dwo,
}
