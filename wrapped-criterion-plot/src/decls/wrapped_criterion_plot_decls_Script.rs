use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Structs that can produce gnuplot code
trait Script {
    /// Translates some configuration struct into gnuplot code
    fn script(&self) -> String;
}
