use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A borrowed trie set.
#[derive(Clone, Copy)]
pub struct TrieSetSlice<'a> {
    /// first tree, one level
    #[doc(hidden)]
    pub tree1_level1: &'a [u64],
    /// second tree, first level
    #[doc(hidden)]
    pub tree2_level1: &'a [u8],
    /// second tree, second level
    #[doc(hidden)]
    pub tree2_level2: &'a [u64],
    /// third tree, first level
    #[doc(hidden)]
    pub tree3_level1: &'a [u8],
    /// third tree, second level
    #[doc(hidden)]
    pub tree3_level2: &'a [u8],
    /// third tree, third level
    #[doc(hidden)]
    pub tree3_level3: &'a [u64],
}
