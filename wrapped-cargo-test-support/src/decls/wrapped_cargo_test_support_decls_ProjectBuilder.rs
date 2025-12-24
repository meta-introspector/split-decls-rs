use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Create a project to run tests against
///
/// - Creates a [`basic_manifest`] if one isn't supplied
///
/// To get started, see:
/// - [`project`]
/// - [`project_in`]
/// - [`project_in_home`]
/// - [`Project::from_template`]
#[must_use]
pub struct ProjectBuilder {
    root: Project,
    files: Vec<FileBuilder>,
    symlinks: Vec<SymlinkBuilder>,
    no_manifest: bool,
}
