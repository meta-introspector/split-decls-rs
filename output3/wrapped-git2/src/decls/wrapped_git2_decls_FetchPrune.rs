use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Configuration for how pruning is done on a fetch
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FetchPrune {
    /// Use the setting from the configuration
    Unspecified,
    /// Force pruning on
    On,
    /// Force pruning off
    Off,
}
