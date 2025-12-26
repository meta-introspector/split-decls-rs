use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl From<EditionedFileId> for HirFileId {
    #[inline]
    fn from(file_id: EditionedFileId) -> Self {
        HirFileId::FileId(file_id)
    }
}
