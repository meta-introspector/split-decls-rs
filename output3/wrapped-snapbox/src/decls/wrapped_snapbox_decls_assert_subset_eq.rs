use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Check if a path matches the content of another path, recursively
///
/// When the content is text, newlines are normalized.
///
/// ```rust,no_run
/// let output_root = "...";
/// let expected_root = "tests/snapshots/output.txt";
/// snapbox::assert_subset_eq(expected_root, output_root);
/// ```
#[cfg(feature = "dir")]
#[track_caller]
pub fn assert_subset_eq(
    expected_root: impl Into<std::path::PathBuf>,
    actual_root: impl Into<std::path::PathBuf>,
) {
    Assert::new()
        .action_env(assert::DEFAULT_ACTION_ENV)
        .subset_eq(expected_root, actual_root);
}
