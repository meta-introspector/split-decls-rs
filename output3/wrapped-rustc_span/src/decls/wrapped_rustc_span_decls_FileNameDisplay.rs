use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub struct FileNameDisplay<'a> {
    inner: &'a FileName,
    display_pref: FileNameDisplayPreference,
}
