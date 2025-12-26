use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Extra information for a future incompatibility lint.
#[derive(Copy, Clone, Debug)]
pub struct FutureIncompatibleInfo {
    /// e.g., a URL for an issue/PR/RFC or error code
    pub reference: &'static str,
    /// The reason for the lint used by diagnostics to provide
    /// the right help message
    pub reason: FutureIncompatibilityReason,
    /// Whether to explain the reason to the user.
    ///
    /// Set to false for lints that already include a more detailed
    /// explanation.
    pub explain_reason: bool,
    /// If set to `true`, this will make future incompatibility warnings show up in cargo's
    /// reports.
    ///
    /// When a future incompatibility warning is first inroduced, set this to `false`
    /// (or, rather, don't override the default). This allows crate developers an opportunity
    /// to fix the warning before blasting all dependents with a warning they can't fix
    /// (dependents have to wait for a new release of the affected crate to be published).
    ///
    /// After a lint has been in this state for a while, consider setting this to true, so it
    /// warns for everyone. It is a good signal that it is ready if you can determine that all
    /// or most affected crates on crates.io have been updated.
    pub report_in_deps: bool,
}
