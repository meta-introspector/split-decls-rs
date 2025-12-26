use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Specification of a single lint.
#[derive(Copy, Clone, Debug)]
pub struct Lint {
    /// A string identifier for the lint.
    ///
    /// This identifies the lint in attributes and in command-line arguments.
    /// In those contexts it is always lowercase, but this field is compared
    /// in a way which is case-insensitive for ASCII characters. This allows
    /// `declare_lint!()` invocations to follow the convention of upper-case
    /// statics without repeating the name.
    ///
    /// The name is written with underscores, e.g., "unused_imports".
    /// On the command line, underscores become dashes.
    ///
    /// See <https://rustc-dev-guide.rust-lang.org/diagnostics.html#lint-naming>
    /// for naming guidelines.
    pub name: &'static str,
    /// Default level for the lint.
    ///
    /// See <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-levels>
    /// for guidelines on choosing a default level.
    pub default_level: Level,
    /// Description of the lint or the issue it detects.
    ///
    /// e.g., "imports that are never used"
    pub desc: &'static str,
    /// Starting at the given edition, default to the given lint level. If this is `None`, then use
    /// `default_level`.
    pub edition_lint_opts: Option<(Edition, Level)>,
    /// `true` if this lint is reported even inside expansions of external macros.
    pub report_in_external_macro: bool,
    pub future_incompatible: Option<FutureIncompatibleInfo>,
    /// `true` if this lint is being loaded by another tool (e.g. Clippy).
    pub is_externally_loaded: bool,
    /// `Some` if this lint is feature gated, otherwise `None`.
    pub feature_gate: Option<Symbol>,
    pub crate_level_only: bool,
    /// `true` if this lint should not be filtered out under any circustamces
    /// (e.g. the unknown_attributes lint)
    pub eval_always: bool,
}
