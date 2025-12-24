use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub mod utils {
    //! Miscellaneous utilities.
    //!
    //! * [`Backoff`], for exponential backoff in spin loops.
    //! * [`CachePadded`], for padding and aligning a value to the length of a cache line.
    pub use crossbeam_utils::Backoff;
    pub use crossbeam_utils::CachePadded;
}
