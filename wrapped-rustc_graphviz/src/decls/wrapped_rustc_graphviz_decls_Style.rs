use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The style for a node or edge.
/// See <https://www.graphviz.org/docs/attr-types/style/> for descriptions.
/// Note that some of these are not valid for edges.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Style {
    None,
    Solid,
    Dashed,
    Dotted,
    Bold,
    Rounded,
    Diagonals,
    Filled,
    Striped,
    Wedged,
}
