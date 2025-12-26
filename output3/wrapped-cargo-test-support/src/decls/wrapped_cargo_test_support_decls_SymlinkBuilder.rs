use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(PartialEq, Clone)]
struct SymlinkBuilder {
    dst: PathBuf,
    src: PathBuf,
    src_is_dir: bool,
}
