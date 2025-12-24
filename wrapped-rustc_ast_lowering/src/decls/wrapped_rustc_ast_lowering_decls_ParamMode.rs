use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Copy, Clone, PartialEq, Debug)]
enum ParamMode {
    /// Any path in a type context.
    Explicit,
    /// The `module::Type` in `module::Type::method` in an expression.
    Optional,
}
