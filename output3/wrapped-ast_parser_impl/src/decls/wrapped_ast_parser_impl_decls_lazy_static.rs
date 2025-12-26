use serde::{Deserialize, Serialize};
use std::collections::HashMap;
lazy_static! {
    static ref TOKIO_RUNTIME: Runtime = Runtime::new().expect("Failed to create Tokio runtime");
}
