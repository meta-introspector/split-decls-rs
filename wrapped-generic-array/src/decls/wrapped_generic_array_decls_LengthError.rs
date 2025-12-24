use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Error type for [`TryFrom`] and [`try_from_iter`](GenericArray::try_from_iter) implementations.
#[derive(Debug, Clone, Copy)]
pub struct LengthError;
