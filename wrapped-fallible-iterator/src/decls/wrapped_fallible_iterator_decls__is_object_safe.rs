use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn _is_object_safe(_: &dyn DoubleEndedFallibleIterator<Item = (), Error = ()>) {}
