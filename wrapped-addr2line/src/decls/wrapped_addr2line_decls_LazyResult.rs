use serde::{Deserialize, Serialize};
use std::collections::HashMap;
type LazyResult<T> = OnceCell<Result<T, Error>>;
