macro_rules! deps {
    () => {
        Trie!();
    };
}

macro_rules! CanonicalCompositions {
    () => {
        deps!();
        # [doc = " Non-Hangul canonical compositions"] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Debug , PartialEq , Clone , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_normalizer :: provider))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] pub struct CanonicalCompositions < 'data > { # [doc = " Trie keys are two-`char` strings with the second"] # [doc = " character coming first. The value, if any, is the"] # [doc = " (non-Hangul) canonical composition."] # [cfg_attr (feature = "serde" , serde (borrow))] pub canonical_compositions : Char16Trie < 'data > , }
    };
}

CanonicalCompositions!();