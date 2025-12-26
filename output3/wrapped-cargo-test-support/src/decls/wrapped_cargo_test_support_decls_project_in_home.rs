use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Generates a project layout inside our fake home dir, see [`ProjectBuilder`]
pub fn project_in_home(name: impl AsRef<Path>) -> ProjectBuilder {
    ProjectBuilder::new(paths::home().join(name))
}
