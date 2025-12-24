use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Labels attached to the tics of an axis
pub struct TicLabels<P, L> {
    /// Labels to attach to the tics
    pub labels: L,
    /// Position of the tics on the axis
    pub positions: P,
}
