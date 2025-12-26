use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Represents the repr options provided by the user.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(
    feature = "nightly",
    derive(Encodable_NoContext, Decodable_NoContext, HashStable_Generic)
)]
pub struct ReprOptions {
    pub int: Option<IntegerType>,
    pub align: Option<Align>,
    pub pack: Option<Align>,
    pub flags: ReprFlags,
    /// The seed to be used for randomizing a type's layout
    ///
    /// Note: This could technically be a `u128` which would
    /// be the "most accurate" hash as it'd encompass the item and crate
    /// hash without loss, but it does pay the price of being larger.
    /// Everything's a tradeoff, a 64-bit seed should be sufficient for our
    /// purposes (primarily `-Z randomize-layout`)
    pub field_shuffle_seed: Hash64,
}
