use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A temporary storage of the pointer.
///
/// This guard object is returned from most loading methods (with the notable exception of
/// [`load_full`](struct.ArcSwapAny.html#method.load_full)). It dereferences to the smart pointer
/// loaded, so most operations are to be done using that.
pub struct Guard<T: RefCnt, S: Strategy<T> = DefaultStrategy> {
    inner: S::Protected,
}
