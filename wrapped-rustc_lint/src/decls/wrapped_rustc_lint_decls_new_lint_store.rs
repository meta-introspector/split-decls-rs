use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub fn new_lint_store(internal_lints: bool) -> LintStore {
    let mut lint_store = LintStore::new();
    register_builtins(&mut lint_store);
    if internal_lints {
        register_internals(&mut lint_store);
    }
    lint_store
}
