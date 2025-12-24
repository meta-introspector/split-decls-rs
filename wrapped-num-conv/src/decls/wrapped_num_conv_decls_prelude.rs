use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Anonymously import all extension traits.
///
/// This allows you to use the methods without worrying about polluting the namespace or importing
/// them individually.
///
/// ```rust
/// use num_conv::prelude::*;
/// ```
pub mod prelude {
    pub use crate::{Extend as _, Truncate as _};
}
