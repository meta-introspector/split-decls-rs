use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Despite this entire module called attribute parsing and the term being a little overloaded,
/// in this module the code lives that actually breaks up tokenstreams into semantic pieces of attributes,
/// like lists or name-value pairs.
pub mod parser;
