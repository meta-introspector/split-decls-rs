use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl LintGroups {
    fn contains(&self, group: &str) -> bool {
        self.groups.contains(&group) || (self.inside_warnings && group == "warnings")
    }
}
