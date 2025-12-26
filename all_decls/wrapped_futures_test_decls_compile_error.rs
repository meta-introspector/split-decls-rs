use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(feature = "std"))]
compile_error!(
    "`futures-test` must have the `std` feature activated, this is a default-active feature"
);
