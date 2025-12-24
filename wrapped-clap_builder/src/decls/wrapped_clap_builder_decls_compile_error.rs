use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(feature = "std"))]
compile_error!("`std` feature is currently required to build `clap`");
