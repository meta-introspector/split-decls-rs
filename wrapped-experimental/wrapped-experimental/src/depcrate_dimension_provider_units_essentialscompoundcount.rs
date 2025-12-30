// Generated macro for CompoundCount (enum)
macro_rules! Depcrate_dimension_provider_units_essentialsCompoundCount {
() => {
// Module: crate::dimension::provider::units::essentials
// Provides: {"CompoundCount"}
// Dependencies: {}
# [doc = " A CLDR plural keyword, or the explicit value 1."] # [doc = " See <https://www.unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules>. // TODO??"] # [zerovec :: make_ule (CompoundCountULE)] # [derive (Copy , Clone , PartialOrd , Ord , PartialEq , Eq , Debug)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_experimental :: dimension :: provider :: units :: essentials))] # [repr (u8)] pub enum CompoundCount { # [doc = " The CLDR keyword `zero`."] Zero = 0 , # [doc = " The CLDR keyword `one`."] One = 1 , # [doc = " The CLDR keyword `two`."] Two = 2 , # [doc = " The CLDR keyword `few`."] Few = 3 , # [doc = " The CLDR keyword `many`."] Many = 4 , # [doc = " The CLDR keyword `other`."] Other = 5 , }
};
}
