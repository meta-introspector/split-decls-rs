// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_session/src/config/cfg.rs
// Error: expected square brackets
// Problematic line: line 36

use crate::config::{CrateType, FmtDebug};
use crate::{Session, errors};

/// The parsed `--cfg` options that define the compilation environment of the
/// crate, used to drive conditional compilation.
///
/// An `FxIndexSet` is used to ensure deterministic ordering of error messages
