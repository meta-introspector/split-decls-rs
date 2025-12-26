use serde::{Deserialize, Serialize};
use std::collections::HashMap;
macro_rules! mkImplCallVisitor {
    (calls : $init_calls:expr) => {
        ImplCallVisitor::new($init_calls)
    };
}
