use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug)]
struct Location {
    line_indent: usize,
    /// The byte range of the argument to `expect!`, including the inner `[]` if it exists.
    literal_range: Range<usize>,
}
