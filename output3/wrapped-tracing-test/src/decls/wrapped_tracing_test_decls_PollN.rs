use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[allow(missing_docs)]
pub struct PollN<T, E> {
    and_return: Option<Result<T, E>>,
    finish_at: usize,
    polls: usize,
}
