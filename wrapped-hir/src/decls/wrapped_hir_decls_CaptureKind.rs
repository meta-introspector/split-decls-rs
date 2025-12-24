use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CaptureKind {
    SharedRef,
    UniqueSharedRef,
    MutableRef,
    Move,
}
