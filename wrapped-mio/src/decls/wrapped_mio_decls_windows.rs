use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(all(windows, feature = "os-ext"))]
#[cfg_attr(docsrs, doc(cfg(all(windows, feature = "os-ext"))))]
pub mod windows {
    //! Windows only extensions.
    pub use crate::sys::named_pipe::NamedPipe;
}
