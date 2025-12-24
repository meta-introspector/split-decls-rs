use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "general")]
pub mod signal_macros {
    pub const SIG_DFL: super::general::__kernel_sighandler_t = None;
    /// Rust doesn't currently permit us to use `transmute` to convert the
    /// `SIG_IGN` value into a function pointer in a `const` initializer, so
    /// we make it a function instead.
    ///
    #[inline]
    pub const fn sig_ign() -> super::general::__kernel_sighandler_t {
        Some(unsafe {
            core::mem::transmute::<usize, unsafe extern "C" fn(crate::ctypes::c_int)>(1)
        })
    }
}
