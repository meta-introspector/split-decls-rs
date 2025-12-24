use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A dummy implementation of `MonsterConstants` for testing.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct DummyMonsterConstants;
