// Generated macro for CollationData (struct)
macro_rules! Depcrate_providerCollationData {
() => {
// Module: crate::provider
// Provides: {"CollationData"}
// Dependencies: {}
# [doc = " The main collation data either for the root or for a tailoring"] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Debug , PartialEq , Clone , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_collator :: provider))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] pub struct CollationData < 'data > { # [doc = " Mapping from `char` to `CollationElement32` (represented"] # [doc = " as its `u32` bits)."] # [cfg_attr (feature = "serde" , serde (borrow))] pub trie : CodePointTrie < 'data , u32 > , # [doc = " `CollationElement`s used in expansions and offset CE32s"] # [doc = " (represented as their `u64` bits)"] # [cfg_attr (feature = "serde" , serde (borrow))] pub ces : ZeroVec < 'data , u64 > , # [doc = " `CollationElement32`s used in expansions and as defaults"] # [doc = " for digits when the numeric mode is not in use"] # [cfg_attr (feature = "serde" , serde (borrow))] pub ce32s : ZeroVec < 'data , u32 > , # [doc = " Defaults and tries for prefix and contraction matching"] # [cfg_attr (feature = "serde" , serde (borrow))] pub contexts : ZeroVec < 'data , u16 > , }
};
}
