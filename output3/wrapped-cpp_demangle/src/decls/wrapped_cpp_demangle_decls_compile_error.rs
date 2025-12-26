use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(feature = "alloc"))]
compile_error!("`alloc` or `std` feature is required for this crate");
