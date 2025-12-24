use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Private
trait ScaleFactorTrait {
    /// Private
    fn scale_factor(&self) -> f64;
}
