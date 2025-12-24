use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(all(target_os = "hermit", feature = "os-ext"))]
#[cfg_attr(docsrs, doc(cfg(all(target_os = "hermit", feature = "os-ext"))))]
pub mod hermit {
    //! Hermit only extensions.
    pub use crate::sys::SourceFd;
}
