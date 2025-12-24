use serde::{Deserialize, Serialize};
use std::collections::HashMap;
struct Solution<'a, 'b> {
    text1: Range<'a>,
    text2: Range<'b>,
    diffs: Vec<Diff<'a, 'b>>,
}
