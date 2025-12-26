use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub struct TempDirBuilder<'a, 'b> {
    builder: tempfile::Builder<'a, 'b>,
}
