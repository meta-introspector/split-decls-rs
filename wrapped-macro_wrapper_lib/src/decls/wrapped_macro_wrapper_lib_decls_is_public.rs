use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Helper to determine if an item is public
fn is_public(vis: &Visibility) -> bool {
    matches!(vis, Visibility::Public(_))
}
