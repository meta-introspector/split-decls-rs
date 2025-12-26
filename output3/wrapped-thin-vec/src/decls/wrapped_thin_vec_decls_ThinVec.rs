use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// See the crate's top level documentation for a description of this type.
#[repr(C)]
pub struct ThinVec<T> {
    ptr: NonNull<Header>,
    boo: PhantomData<T>,
}
