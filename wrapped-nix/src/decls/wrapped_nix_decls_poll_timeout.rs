use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(any(feature = "poll", feature = "event"))]
mod poll_timeout;
