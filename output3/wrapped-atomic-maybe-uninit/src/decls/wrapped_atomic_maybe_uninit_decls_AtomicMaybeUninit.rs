use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A potentially uninitialized integer type which can be safely shared between threads.
///
/// This type has the same in-memory representation as the underlying
/// value type, `MaybeUninit<T>`.
#[repr(C)]
pub struct AtomicMaybeUninit<T: Primitive> {
    v: UnsafeCell<MaybeUninit<T>>,
    /// `[T::Align; 0]` ensures alignment is at least that of `T::Align`.
    ///
    /// This is needed because x86's u64 is 4-byte aligned and x86_64's u128 is
    /// 8-byte aligned and atomic operations normally require alignment greater
    /// than or equal to the size.
    _align: [T::Align; 0],
}
