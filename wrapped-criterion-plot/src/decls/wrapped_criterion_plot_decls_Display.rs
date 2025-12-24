use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Enums that can produce gnuplot code
trait Display<S> {
    /// Translates the enum in gnuplot code
    fn display(&self) -> S;
}
