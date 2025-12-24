use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Grid line
#[derive(Clone, Copy)]
pub enum Grid {
    /// Major gridlines
    Major,
    /// Minor gridlines
    Minor,
}
