use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Chunk<'a> {
    Equal(&'a str),
    Delete(&'a str),
    Insert(&'a str),
}
