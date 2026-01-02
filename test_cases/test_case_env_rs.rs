// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/env.rs
// Error: expected square brackets
// Problematic line: line 21

use crate::sys::{env as env_imp, os as os_imp};
use crate::{array, fmt, io, sys};

/// Returns the current working directory as a [`PathBuf`].
///
/// # Platform-specific behavior
///
