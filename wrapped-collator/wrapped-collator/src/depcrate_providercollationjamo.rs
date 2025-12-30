// Generated macro for CollationJamo (struct)
macro_rules! Depcrate_providerCollationJamo {
() => {
// Module: crate::provider
// Provides: {"CollationJamo"}
// Dependencies: {}
# [doc = " `CollationElement32`s for the Hangul Jamo Unicode Block"] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Debug , PartialEq , Clone , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_collator :: provider))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] pub struct CollationJamo < 'data > { # [doc = " `CollationElement32`s (as `u32`s) for the Hangul Jamo Unicode Block."] # [doc = " The length must be equal to the size of the block (256)."] # [cfg_attr (feature = "serde" , serde (borrow))] pub ce32s : ZeroVec < 'data , u32 > , }
};
}
