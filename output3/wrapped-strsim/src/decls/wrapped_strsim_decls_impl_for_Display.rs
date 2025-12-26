use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Display for StrSimError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        let text = match self {
            StrSimError::DifferentLengthArgs => "Differing length arguments provided",
        };
        write!(fmt, "{}", text)
    }
}
