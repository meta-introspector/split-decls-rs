// Generated macro for CollationMetadata (struct)
macro_rules! Depcrate_providerCollationMetadata {
() => {
// Module: crate::provider
// Provides: {"CollationMetadata"}
// Dependencies: {}
# [doc = " Each non-alias collation that the data provider knows"] # [doc = " about explicitly has an data entry at least for this"] # [doc = " struct."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Debug , PartialEq , Clone , Copy , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_collator :: provider))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] pub struct CollationMetadata { # [doc = " See the mask constants in the `impl` block for the"] # [doc = " bit layout. The other bits are ignored: They could"] # [doc = " be from the future if their semantics such that"] # [doc = " old code may ignore them."] # [doc = ""] # [doc = " Note: At present, it's bogus for the bit for \"upper"] # [doc = " first\" to be set if \"case first\" isn't also set."] # [doc = " However, the methods handle this case gracefully,"] # [doc = " so there is no need for invariant validation."] pub bits : u32 , }
};
}
