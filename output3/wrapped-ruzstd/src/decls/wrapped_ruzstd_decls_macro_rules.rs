use serde::{Deserialize, Serialize};
use std::collections::HashMap;
macro_rules! vprintln {
    ($($x:expr),*) => {
        #[cfg(feature = "std")] if crate ::VERBOSE { std::println!($($x),*); }
    };
}
