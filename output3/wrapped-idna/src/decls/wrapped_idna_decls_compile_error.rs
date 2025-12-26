use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(feature = "compiled_data"))]
compile_error!("the `compiled_data` feature must be enabled");
