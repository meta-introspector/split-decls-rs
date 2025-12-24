use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An iterator that unwraps every element yielded by the underlying
/// FallibleIterator
#[derive(Clone, Debug)]
pub struct Unwrap<T>(T);
