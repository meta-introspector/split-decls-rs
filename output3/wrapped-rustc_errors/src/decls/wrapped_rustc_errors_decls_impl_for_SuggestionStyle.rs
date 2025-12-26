use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl SuggestionStyle {
    fn hide_inline(&self) -> bool {
        !matches!(*self, SuggestionStyle::ShowCode)
    }
}
