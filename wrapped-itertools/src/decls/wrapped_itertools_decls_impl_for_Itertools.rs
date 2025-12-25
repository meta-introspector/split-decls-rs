use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T> Itertools for T where T: Iterator + ?Sized {}
