use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The level at which the tracing item should be created.
///
/// It's used to filter items early.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum Level {
    /// A coarse-grained trace level, one that should span entire operations with low frequency.
    Coarse = 1,
    /// Finer grained trace level that further subdivides coarse-level traces.
    ///
    /// Note that these should only be created for areas of the code which have significant cost.
    Detail = 2,
}
