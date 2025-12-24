use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "digest-scheme")]
#[cfg_attr(docsrs, doc(cfg(feature = "digest-scheme")))]
pub mod digest;
