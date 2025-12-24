use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Fill potentially uninitialized buffer `dest` with random bytes from
/// the system's preferred random number source and return a mutable
/// reference to those bytes.
///
/// On successful completion this function is guaranteed to return a slice
/// which points to the same memory as `dest` and has the same length.
/// In other words, it's safe to assume that `dest` is initialized after
/// this function has returned `Ok`.
///
/// No part of `dest` will ever be de-initialized at any point, regardless
/// of what is returned.
///
/// # Examples
///
/// ```ignore
/// # // We ignore this test since `uninit_array` is unstable.
/// #![feature(maybe_uninit_uninit_array)]
/// # fn main() -> Result<(), getrandom::Error> {
/// let mut buf = core::mem::MaybeUninit::uninit_array::<1024>();
/// let buf: &mut [u8] = getrandom::fill_uninit(&mut buf)?;
/// # Ok(()) }
/// ```
#[inline]
pub fn fill_uninit(dest: &mut [MaybeUninit<u8>]) -> Result<&mut [u8], Error> {
    if !dest.is_empty() {
        backends::fill_inner(dest)?;
    }
    #[cfg(getrandom_msan)]
    unsafe extern "C" {
        fn __msan_unpoison(a: *mut core::ffi::c_void, size: usize);
    }
    Ok(unsafe { util::slice_assume_init_mut(dest) })
}
