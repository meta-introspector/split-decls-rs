// Generated macro for LanguageStrStrPair (struct)
macro_rules! Depcrate_providerLanguageStrStrPair {
() => {
// Module: crate::provider
// Provides: {"LanguageStrStrPair"}
// Dependencies: {}
# [zerovec :: make_varule (LanguageStrStrPairVarULE)] # [zerovec :: derive (Debug)] # [derive (Clone , PartialEq , Eq , PartialOrd , Ord , Debug)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize) , zerovec :: derive (Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake) , zerovec :: derive (Serialize) , databake (path = icu_locale :: provider) ,)] # [doc = " A triplet of strings with a EncodeAsVarULE implementation."] pub struct LanguageStrStrPair < 'a > (pub Language , # [cfg_attr (feature = "serde" , serde (borrow))] pub Cow < 'a , str > , # [cfg_attr (feature = "serde" , serde (borrow))] pub Cow < 'a , str > ,) ;
};
}
