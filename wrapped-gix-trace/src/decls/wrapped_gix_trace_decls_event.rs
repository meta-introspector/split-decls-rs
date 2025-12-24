use serde::{Deserialize, Serialize};
use std::collections::HashMap;
///
pub mod event {
    #[cfg(feature = "tracing")]
    pub use tracing_core::Level;
    /// All available tracing levels for use in `event!()` macro.
    #[cfg(not(feature = "tracing"))]
    #[repr(usize)]
    #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
    pub enum Level {
        /// The "trace" level.
        ///
        /// Designates very low priority, often extremely verbose, information.
        TRACE = 0,
        /// The "debug" level.
        ///
        /// Designates lower priority information.
        DEBUG = 1,
        /// The "info" level.
        ///
        /// Designates useful information.
        INFO = 2,
        /// The "warn" level.
        ///
        /// Designates hazardous situations.
        WARN = 3,
        /// The "error" level.
        ///
        /// Designates very serious errors.
        ERROR = 4,
    }
}
