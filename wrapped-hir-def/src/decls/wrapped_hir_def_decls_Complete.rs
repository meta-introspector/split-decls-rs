use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// `#[rust_analyzer::completions(...)]` options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Complete {
    /// No `#[rust_analyzer::completions(...)]`.
    Yes,
    /// `#[rust_analyzer::completions(ignore_flyimport)]`.
    IgnoreFlyimport,
    /// `#[rust_analyzer::completions(ignore_flyimport_methods)]` (on a trait only).
    IgnoreFlyimportMethods,
    /// `#[rust_analyzer::completions(ignore_methods)]` (on a trait only).
    IgnoreMethods,
}
