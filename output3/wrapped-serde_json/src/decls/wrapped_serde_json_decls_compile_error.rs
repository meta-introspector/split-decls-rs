use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error! {
    "serde_json requires that either `std` (default) or `alloc` feature is enabled"
}
