use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Curve variant of Default
trait CurveDefault<S> {
    /// Creates `curve::Properties` with default configuration
    fn default(s: S) -> Self;
}
