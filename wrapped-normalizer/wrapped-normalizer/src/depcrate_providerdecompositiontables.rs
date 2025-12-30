// Generated macro for DecompositionTables (struct)
macro_rules! Depcrate_providerDecompositionTables {
() => {
// Module: crate::provider
// Provides: {"DecompositionTables"}
// Dependencies: {}
# [doc = " The expansion tables for cases where the decomposition isn't"] # [doc = " contained in the trie value"] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Debug , PartialEq , Clone , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_normalizer :: provider))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] pub struct DecompositionTables < 'data > { # [doc = " Decompositions that are fully within the BMP"] # [cfg_attr (feature = "serde" , serde (borrow))] pub scalars16 : ZeroVec < 'data , u16 > , # [doc = " Decompositions with at least one character outside"] # [doc = " the BMP"] # [cfg_attr (feature = "serde" , serde (borrow))] pub scalars24 : ZeroVec < 'data , char > , }
};
}
