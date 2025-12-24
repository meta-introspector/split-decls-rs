use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An Xorshift random number generator.
///
/// The Xorshift[^1] algorithm is not suitable for cryptographic purposes
/// but is very fast. If you do not know for sure that it fits your
/// requirements, use a more secure one such as `StdRng` or `OsRng`.
///
/// When seeded with zero (i.e. `XorShiftRng::from_seed(0)` is called), this implementation
/// actually uses `0xBAD_5EED_0BAD_5EED_0BAD_5EED_0BAD_5EED` for the seed. This arbitrary value is
/// used because the underlying algorithm can't escape from an all-zero state, and the function is
/// infallible so it can't signal this by returning an error.
///
/// [^1]: Marsaglia, George (July 2003).
///       ["Xorshift RNGs"](https://www.jstatsoft.org/v08/i14/paper).
///       *Journal of Statistical Software*. Vol. 8 (Issue 14).
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct XorShiftRng {
    x: w<u32>,
    y: w<u32>,
    z: w<u32>,
    w: w<u32>,
}
