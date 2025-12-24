use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A set of cfg-overrides per crate.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct CfgOverrides {
    /// A global set of overrides matching all crates.
    pub global: cfg::CfgDiff,
    /// A set of overrides matching specific crates.
    pub selective: rustc_hash::FxHashMap<String, cfg::CfgDiff>,
}
