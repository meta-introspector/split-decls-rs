use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A correctly sized stack allocation for the formatted integer to be written
/// into.
///
/// # Example
///
/// ```
/// let mut buffer = itoa::Buffer::new();
/// let printed = buffer.format(1234);
/// assert_eq!(printed, "1234");
/// ```
pub struct Buffer {
    bytes: [MaybeUninit<u8>; i128::MAX_STR_LEN],
}
