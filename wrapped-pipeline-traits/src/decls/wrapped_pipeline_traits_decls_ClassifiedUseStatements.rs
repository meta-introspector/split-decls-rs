use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug)]
pub struct ClassifiedUseStatements(pub Vec<UseStatement>, pub HashMap<String, Vec<String>>);
