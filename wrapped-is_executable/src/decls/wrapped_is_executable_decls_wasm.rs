use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(any(target_os = "wasi", target_family = "wasm"))]
mod wasm {
    use super::IsExecutable;
    use std::path::Path;
    impl IsExecutable for Path {
        fn is_executable(&self) -> bool {
            false
        }
    }
}
