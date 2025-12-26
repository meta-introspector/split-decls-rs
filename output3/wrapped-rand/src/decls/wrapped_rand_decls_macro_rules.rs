use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[allow(unused)]
macro_rules! error {
    ($($x:tt)*) => {
        #[cfg(feature = "log")] { log::error!($($x)*) }
    };
}
