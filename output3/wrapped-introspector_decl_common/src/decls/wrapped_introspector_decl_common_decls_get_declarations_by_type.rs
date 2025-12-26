use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub fn get_declarations_by_type(node_type: &str) -> Vec<DeclInfo> {
    DECL_REGISTRY
        .lock()
        .map(|r| {
            r.by_type
                .get(node_type)
                .map(|indices| {
                    indices
                        .iter()
                        .filter_map(|&i| r.declarations.get(i).cloned())
                        .collect()
                })
                .unwrap_or_default()
        })
        .unwrap_or_default()
}
