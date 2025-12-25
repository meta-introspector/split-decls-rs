use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "alloc")]
#[inline(always)]
fn invalid_mut<T>(addr: usize) -> *mut T {
    #[allow(clippy::useless_transmute, clippy::transmutes_expressible_as_ptr_casts)]
    unsafe {
        core::mem::transmute(addr)
    }
}
