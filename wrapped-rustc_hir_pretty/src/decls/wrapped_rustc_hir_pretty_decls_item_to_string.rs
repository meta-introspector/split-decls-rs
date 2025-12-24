use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub fn item_to_string(ann: &dyn PpAnn, pat: &hir::Item<'_>) -> String {
    to_string(ann, |s| s.print_item(pat))
}
