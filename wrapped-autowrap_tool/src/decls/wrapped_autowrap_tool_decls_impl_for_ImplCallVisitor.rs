use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl ImplCallVisitor {
    fn new(
        init_calls: std::collections::HashMap<String, std::collections::HashSet<String>>,
    ) -> Self {
        ImplCallVisitor { calls: init_calls }
    }
}
