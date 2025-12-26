use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A wrapper around a writer that finishes the stream on drop.
#[allow(private_bounds)]
pub struct AutoFinisher<T: AutoFinish>(Option<T>);
