use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl From<&cstore::NativeLib> for NativeLib {
    fn from(lib: &cstore::NativeLib) -> Self {
        NativeLib {
            kind: lib.kind,
            filename: lib.filename,
            name: lib.name,
            cfg: lib.cfg.clone(),
            verbatim: lib.verbatim.unwrap_or(false),
            dll_imports: lib.dll_imports.clone(),
        }
    }
}
