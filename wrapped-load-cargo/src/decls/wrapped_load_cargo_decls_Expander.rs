use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, PartialEq, Eq)]
struct Expander(proc_macro_api::ProcMacro);
