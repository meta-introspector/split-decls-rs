use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "thread-pool")]
#[cfg_attr(docsrs, doc(cfg(feature = "thread-pool")))]
#[cfg(feature = "std")]
mod thread_pool;
