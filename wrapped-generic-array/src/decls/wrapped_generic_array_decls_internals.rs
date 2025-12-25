use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "internals")]
pub mod internals {
    //! Very unsafe internal functionality.
    //!
    //! These are used internally for building and consuming generic arrays. When used correctly,
    //! they can ensure elements are correctly dropped if something panics while using them.
    //!
    //! The API of these is not guaranteed to be stable, as they are not intended for general use.
    pub use crate::internal::{ArrayBuilder, ArrayConsumer};
    pub use crate::internal::{IntrusiveArrayBuilder, IntrusiveArrayConsumer};
}
