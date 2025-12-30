// Generated macro for CollationDiacritics (struct)
macro_rules! Depcrate_providerCollationDiacritics {
() => {
// Module: crate::provider
// Provides: {"CollationDiacritics"}
// Dependencies: {}
# [doc = " Secondary weights for the start of the Combining Diacritics block."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Debug , PartialEq , Clone , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_collator :: provider))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] pub struct CollationDiacritics < 'data > { # [doc = " Secondary weights for characters starting from U+0300 up"] # [doc = " to but not including U+034F. May be shorter than that;"] # [doc = " zero-length when a tailoring opts out of using this"] # [doc = " feature altogether."] # [cfg_attr (feature = "serde" , serde (borrow))] pub secondaries : ZeroVec < 'data , u16 > , }
};
}
