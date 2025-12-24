use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Error bar variant of Default
trait ErrorBarDefault<S> {
    /// Creates `errorbar::Properties` with default configuration
    fn default(s: S) -> Self;
}
