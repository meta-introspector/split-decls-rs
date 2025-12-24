use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Fill `dest` with random bytes from the system's preferred random number source.
///
/// This function returns an error on any failure, including partial reads. We
/// make no guarantees regarding the contents of `dest` on error. If `dest` is
/// empty, `getrandom` immediately returns success, making no calls to the
/// underlying operating system.
///
/// Blocking is possible, at least during early boot; see module documentation.
///
/// In general, `getrandom` will be fast enough for interactive usage, though
/// significantly slower than a user-space CSPRNG; for the latter consider
/// [`rand::thread_rng`](https://docs.rs/rand/*/rand/fn.thread_rng.html).
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), getrandom::Error> {
/// let mut buf = [0u8; 32];
/// getrandom::fill(&mut buf)?;
/// # Ok(()) }
/// ```
#[inline]
pub fn fill(dest: &mut [u8]) -> Result<(), Error> {
    fill_uninit(unsafe { util::slice_as_uninit_mut(dest) })?;
    Ok(())
}
