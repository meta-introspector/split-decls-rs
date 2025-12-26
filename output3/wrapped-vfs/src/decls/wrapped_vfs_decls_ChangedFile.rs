use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Changed file in the [`Vfs`].
#[derive(Debug)]
pub struct ChangedFile {
    /// Id of the changed file
    pub file_id: FileId,
    /// Kind of change
    pub change: Change,
}
