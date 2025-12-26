use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl fmt::Debug for OutputReader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("OutputReader")
            .field("position", &self.position())
            .finish()
    }
}
