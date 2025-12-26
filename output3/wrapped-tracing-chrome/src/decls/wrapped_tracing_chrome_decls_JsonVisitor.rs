use serde::{Deserialize, Serialize};
use std::collections::HashMap;
struct JsonVisitor<'a> {
    object: &'a mut Object,
}
