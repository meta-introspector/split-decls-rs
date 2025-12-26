use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Case Insensitive wrapper of Ascii strings.
#[derive(Clone, Copy, Debug, Default)]
pub struct Ascii<S>(S);
