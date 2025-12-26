use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Clone, Copy)]
enum StrLitKind {
    Normal,
    Raw(usize),
}
