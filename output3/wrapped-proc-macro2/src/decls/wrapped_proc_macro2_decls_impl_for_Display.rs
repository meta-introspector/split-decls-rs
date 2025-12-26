use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}
