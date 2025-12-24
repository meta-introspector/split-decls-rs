use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A speedy hash algorithm for use within rustc. The hashmap in liballoc
/// by default uses SipHash which isn't quite as speedy as we want. In the
/// compiler we're not really worried about DOS attempts, so we use a fast
/// non-cryptographic hash.
///
/// The current implementation is a fast polynomial hash with a single
/// bit rotation as a finishing step designed by Orson Peters.
#[derive(Clone)]
pub struct FxHasher {
    hash: usize,
}
