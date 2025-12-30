// Generated macro for Count (enum)
macro_rules! Depcrate_compactdecimal_providerCount {
() => {
// Module: crate::compactdecimal::provider
// Provides: {"Count"}
// Dependencies: {}
# [doc = " A CLDR plural keyword, or the explicit value 1."] # [doc = " See <https://www.unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules>."] # [zerovec :: make_ule (CountULE)] # [zerovec :: derive (Debug)] # [derive (Copy , Clone , PartialOrd , Ord , PartialEq , Eq , Debug)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_experimental :: compactdecimal :: provider))] # [repr (u8)] pub enum Count { # [doc = " The CLDR keyword `zero`."] Zero = 0 , # [doc = " The CLDR keyword `one`."] One = 1 , # [doc = " The CLDR keyword `two`."] Two = 2 , # [doc = " The CLDR keyword `few`."] Few = 3 , # [doc = " The CLDR keyword `many`."] Many = 4 , # [doc = " The CLDR keyword `other`."] Other = 5 , # [doc = " The explicit 1 case, see <https://www.unicode.org/reports/tr35/tr35-numbers.html#Explicit_0_1_rules>."] Explicit1 = 6 , }
};
}
