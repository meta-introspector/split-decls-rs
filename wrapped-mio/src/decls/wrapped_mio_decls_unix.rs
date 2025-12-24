use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(all(unix, feature = "os-ext"))]
#[cfg_attr(docsrs, doc(cfg(all(unix, feature = "os-ext"))))]
pub mod unix {
    //! Unix only extensions.
    pub mod pipe {
        //! Unix pipe.
        //!
        //! See the [`new`] function for documentation.
        pub use crate::sys::pipe::{new, Receiver, Sender};
    }
    pub use crate::sys::SourceFd;
}
