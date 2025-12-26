use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ValueResult<T, E> {
    pub value: T,
    pub err: Option<E>,
}
