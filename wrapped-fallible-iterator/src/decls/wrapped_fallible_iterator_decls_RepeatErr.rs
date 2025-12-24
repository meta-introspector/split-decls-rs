use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An iterator that endlessly repeats a single error.
#[derive(Clone, Debug)]
pub struct RepeatErr<T, E: Clone>(PhantomData<T>, E);
