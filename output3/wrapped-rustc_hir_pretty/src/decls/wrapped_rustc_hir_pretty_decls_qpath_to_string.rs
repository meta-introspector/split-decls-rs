use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub fn qpath_to_string(ann: &dyn PpAnn, segment: &hir::QPath<'_>) -> String {
    to_string(ann, |s| s.print_qpath(segment, false))
}
