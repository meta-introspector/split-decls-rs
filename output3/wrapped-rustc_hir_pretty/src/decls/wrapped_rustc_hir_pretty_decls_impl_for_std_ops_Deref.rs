use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl std::ops::Deref for State<'_> {
    type Target = pp::Printer;
    fn deref(&self) -> &Self::Target {
        &self.s
    }
}
