use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// We need to transmit raw pointers across threads. It is possible to do this
/// without any unsafe code by converting pointers to usize or to AtomicPtr<T>
/// then back to a raw pointer for use. We prefer this approach because code
/// that uses this type is more explicit.
///
/// Unsafe code is still required to dereference the pointer, so this type is
/// not unsound on its own, although it does partly lift the unconditional
/// !Send and !Sync on raw pointers. As always, dereference with care.
struct SendPtr<T>(*mut T);
