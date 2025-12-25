use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Helper trait to hide the complex bound under a simpler name
trait IsWithinUsizeBound: typenum::IsLess<MaxArrayLengthP1, Output = typenum::consts::True> {}
