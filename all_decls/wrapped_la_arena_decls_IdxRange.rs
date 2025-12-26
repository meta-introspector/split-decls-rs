use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A range of densely allocated arena values.
pub struct IdxRange<T> {
    range: Range<u32>,
    _p: PhantomData<T>,
}
