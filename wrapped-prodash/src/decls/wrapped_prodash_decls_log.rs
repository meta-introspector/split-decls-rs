use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(feature = "progress-tree-log"))]
mod log {
    /// Stub
    #[macro_export(local_inner_macros)]
    macro_rules! warn {
        (target : $target:expr, $($arg:tt)+) => {};
        ($($arg:tt)+) => {};
    }
    /// Stub
    #[macro_export(local_inner_macros)]
    macro_rules! info {
        (target : $target:expr, $($arg:tt)+) => {};
        ($($arg:tt)+) => {};
    }
}
