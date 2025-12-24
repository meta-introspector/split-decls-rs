use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "ffi")]
#[cfg_attr(docsrs, doc(cfg(all(feature = "ffi", hyper_unstable_ffi))))]
pub mod ffi;
