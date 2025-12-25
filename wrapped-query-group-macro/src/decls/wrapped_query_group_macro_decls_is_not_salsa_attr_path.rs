use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn is_not_salsa_attr_path(path: &syn::Path) -> bool {
    path.segments.first().map(|s| s.ident != "salsa").unwrap_or(true)
        || path.segments.len() != 2
}
