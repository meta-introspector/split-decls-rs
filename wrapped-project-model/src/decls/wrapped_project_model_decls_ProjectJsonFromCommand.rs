use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectJsonFromCommand {
    /// The data describing this project, such as its dependencies.
    pub data: ProjectJsonData,
    /// The build system specific file that describes this project,
    /// such as a `my-project/BUCK` file.
    pub buildfile: AbsPathBuf,
}
