use serde::{Deserialize, Serialize};
use std::collections::HashMap;
struct Entry<T> {
    present: AtomicBool,
    value: UnsafeCell<MaybeUninit<T>>,
}
