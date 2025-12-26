use serde::{Deserialize, Serialize};
use std::collections::HashMap;
struct ImplCallVisitor {
    calls: std::collections::HashMap<String, std::collections::HashSet<String>>,
}
