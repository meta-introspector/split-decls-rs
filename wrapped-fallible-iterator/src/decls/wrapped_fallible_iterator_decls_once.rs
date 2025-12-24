use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Creates an iterator that yields an element exactly once.
pub fn once<T, E>(value: T) -> Once<T, E> {
    Once(Some(value), PhantomData)
}
