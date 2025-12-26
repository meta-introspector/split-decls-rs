use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A cargo project to run tests against.
///
/// See [`ProjectBuilder`] or [`Project::from_template`] to get started.
pub struct Project {
    root: PathBuf,
}
