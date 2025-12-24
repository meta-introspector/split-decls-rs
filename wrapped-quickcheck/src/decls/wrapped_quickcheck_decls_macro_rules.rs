use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(feature = "use_logging"))]
macro_rules! info {
    ($($_ignore:tt)*) => {
        ()
    };
}
