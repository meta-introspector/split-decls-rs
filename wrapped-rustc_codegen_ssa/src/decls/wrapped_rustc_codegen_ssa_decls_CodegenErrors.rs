use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub enum CodegenErrors {
    WrongFileType,
    EmptyVersionNumber,
    EncodingVersionMismatch {
        version_array: String,
        rlink_version: u32,
    },
    RustcVersionMismatch {
        rustc_version: String,
    },
    CorruptFile,
}
