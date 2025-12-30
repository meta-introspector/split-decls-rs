// Generated macro for PersonNamesFormattingData (struct)
macro_rules! Depcrate_personnames_providerPersonNamesFormattingData {
() => {
// Module: crate::personnames::provider
// Provides: {"PersonNamesFormattingData"}
// Dependencies: {}
# [doc = " PersonName Formatting data."] # [doc = ""] # [doc = " <https://www.unicode.org/reports/tr35/tr35-personNames.html#personname-element>"] # [zerovec :: make_varule (PersonNamesFormattingDataVarULE)] # [zerovec :: skip_derive (ZeroMapKV , Ord)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_experimental :: personnames :: provider))] # [cfg_attr (feature = "datagen" , zerovec :: derive (Serialize))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize) , zerovec :: derive (Deserialize))] pub struct PersonNamesFormattingData < 'data > { # [doc = " Attributes"] # [doc = " <https://www.unicode.org/reports/tr35/tr35-personNames.html#personname-element>"] pub attributes : PersonNamesFormattingAttributesMask , # [doc = " <https://www.unicode.org/reports/tr35/tr35-personNames.html#namepattern-syntax>"] # [cfg_attr (feature = "serde" , serde (borrow))] pub patterns : VarZeroVec < 'data , str > , }
};
}
