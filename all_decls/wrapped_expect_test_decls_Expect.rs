use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Self-updating string literal.
#[derive(Debug)]
pub struct Expect {
    #[doc(hidden)]
    pub position: Position,
    #[doc(hidden)]
    pub data: &'static str,
    #[doc(hidden)]
    pub indent: bool,
}
