use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub fn expr_to_string(ann: &dyn PpAnn, pat: &hir::Expr<'_>) -> String {
    to_string(ann, |s| s.print_expr(pat))
}
