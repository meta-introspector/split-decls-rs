use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The movability of a coroutine / closure literal:
/// whether a coroutine contains self-references, causing it to be `!Unpin`.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Copy)]
#[cfg_attr(
    feature = "nightly",
    derive(Encodable_NoContext, Decodable_NoContext, HashStable_NoContext)
)]
pub enum Movability {
    /// May contain self-references, `!Unpin`.
    Static,
    /// Must not contain self-references, `Unpin`.
    Movable,
}
