use serde::{Deserialize, Serialize};
use std::collections::HashMap;
unsafe impl<T: Primitive> Sync for AtomicMaybeUninit<T> {}
