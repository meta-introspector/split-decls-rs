use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
#[cfg_attr(
    feature = "nightly",
    derive(Encodable_NoContext, Decodable_NoContext, HashStable_NoContext)
)]
pub enum UintTy {
    Usize,
    U8,
    U16,
    U32,
    U64,
    U128,
}
