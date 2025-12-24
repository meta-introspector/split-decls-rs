use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct Feature {
    pub name: Symbol,
    /// For unstable features: the version the feature was added in.
    /// For accepted features: the version the feature got stabilized in.
    /// For removed features we are inconsistent; sometimes this is the
    /// version it got added, sometimes the version it got removed.
    pub since: &'static str,
    issue: Option<NonZero<u32>>,
}
