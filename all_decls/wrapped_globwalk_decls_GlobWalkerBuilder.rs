use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An iterator for recursively yielding glob matches.
///
/// The order of elements yielded by this iterator is unspecified.
pub struct GlobWalkerBuilder {
    root: PathBuf,
    patterns: Vec<String>,
    walker: WalkDir,
    case_insensitive: bool,
    file_type: Option<FileType>,
}
