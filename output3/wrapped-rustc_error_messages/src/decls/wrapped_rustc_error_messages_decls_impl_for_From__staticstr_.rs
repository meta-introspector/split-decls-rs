use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl From<&'static str> for DiagMessage {
    fn from(s: &'static str) -> Self {
        DiagMessage::Str(Cow::Borrowed(s))
    }
}
