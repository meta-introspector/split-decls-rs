use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Clone, Copy, Debug, Hash)]
pub enum UnstableFeatures {
    /// Disallow use of unstable features, as on beta/stable channels.
    Disallow,
    /// Allow use of unstable features, as on nightly.
    Allow,
    /// Errors are bypassed for bootstrapping. This is required any time
    /// during the build that feature-related lints are set to warn or above
    /// because the build turns on warnings-as-errors and uses lots of unstable
    /// features. As a result, this is always required for building Rust itself.
    Cheat,
}
