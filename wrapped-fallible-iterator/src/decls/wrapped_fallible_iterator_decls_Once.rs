use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An iterator that yields something exactly once.
#[derive(Clone, Debug)]
pub struct Once<T, E>(Option<T>, PhantomData<E>);
