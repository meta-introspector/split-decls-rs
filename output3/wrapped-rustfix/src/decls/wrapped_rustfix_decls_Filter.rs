use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A filter to control which suggestion should be applied.
#[derive(Debug, Clone, Copy)]
pub enum Filter {
    /// For [`diagnostics::Applicability::MachineApplicable`] only.
    MachineApplicableOnly,
    /// Everything is included. YOLO!
    Everything,
}
