// Generated macro for StrStrPair (struct)
macro_rules! Depcrate_providerStrStrPair {
() => {
// Module: crate::provider
// Provides: {"StrStrPair"}
// Dependencies: {}
# [zerovec :: make_varule (StrStrPairVarULE)] # [zerovec :: derive (Debug)] # [derive (Clone , PartialEq , Eq , PartialOrd , Ord , Debug)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize) , zerovec :: derive (Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake) , zerovec :: derive (Serialize) , databake (path = icu_locale :: provider) ,)] # [doc = " A pair of strings with a EncodeAsVarULE implementation."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] pub struct StrStrPair < 'a > (# [cfg_attr (feature = "serde" , serde (borrow))] pub Cow < 'a , str > , # [cfg_attr (feature = "serde" , serde (borrow))] pub Cow < 'a , str > ,) ;
};
}
