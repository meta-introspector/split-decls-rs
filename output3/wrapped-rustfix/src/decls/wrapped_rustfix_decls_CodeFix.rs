use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Represents a code fix. This doesn't write to disks but is only in memory.
///
/// The general way to use this is:
///
/// 1. Feeds the source of a file to [`CodeFix::new`].
/// 2. Calls [`CodeFix::apply`] to apply suggestions to the source code.
/// 3. Calls [`CodeFix::finish`] to get the "fixed" code.
#[derive(Clone)]
pub struct CodeFix {
    data: replace::Data,
    /// Whether or not the data has been modified.
    modified: bool,
}
