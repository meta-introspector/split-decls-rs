use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T: Send> Default for ThreadLocal<T> {
    fn default() -> ThreadLocal<T> {
        ThreadLocal::new()
    }
}
