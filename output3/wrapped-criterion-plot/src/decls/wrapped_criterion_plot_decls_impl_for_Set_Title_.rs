use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Set<Title> for Figure {
    /// Sets the title
    fn set(&mut self, title: Title) -> &mut Figure {
        self.title = Some(title.0);
        self
    }
}
