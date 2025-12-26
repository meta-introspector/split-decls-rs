use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// This is a marker for a fatal compiler error used with `resume_unwind`.
pub struct FatalErrorMarker;
