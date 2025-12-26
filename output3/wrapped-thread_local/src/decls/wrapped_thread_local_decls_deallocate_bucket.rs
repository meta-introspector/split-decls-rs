use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// # Safety
/// The caller must ensure that `bucket` was allocated from [allocate_bucket]
/// with the same `size` parameter.
unsafe fn deallocate_bucket<T>(bucket: *mut Entry<T>, size: usize) {
    let slice = unsafe { std::slice::from_raw_parts_mut(bucket, size) };
    drop(unsafe { Box::from_raw(slice) });
}
