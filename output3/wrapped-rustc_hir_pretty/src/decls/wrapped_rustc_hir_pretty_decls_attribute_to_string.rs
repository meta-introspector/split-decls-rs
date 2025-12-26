use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub fn attribute_to_string(ann: &dyn PpAnn, attr: &hir::Attribute) -> String {
    to_string(ann, |s| {
        s.print_attribute_as_style(attr, ast::AttrStyle::Outer)
    })
}
