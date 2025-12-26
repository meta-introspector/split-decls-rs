use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl fmt::Debug for EditionedFileId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("EditionedFileId")
            .field(&self.file_id().index())
            .field(&self.edition())
            .finish()
    }
}
