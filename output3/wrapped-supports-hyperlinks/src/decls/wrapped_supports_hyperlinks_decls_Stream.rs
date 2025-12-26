use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// possible stream sources
#[derive(Clone, Copy, Debug)]
pub enum Stream {
    Stdout,
    Stderr,
}
