// Generated macro for PatternSelection (enum)
macro_rules! Depcrate_dimension_provider_currency_essentialsPatternSelection {
() => {
// Module: crate::dimension::provider::currency::essentials
// Provides: {"PatternSelection"}
// Dependencies: {}
# [zerovec :: make_ule (PatternSelectionULE)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_experimental :: dimension :: provider :: currency :: essentials))] # [derive (Copy , Clone , Debug , PartialOrd , Ord , PartialEq , Eq , Default)] # [repr (u8)] pub enum PatternSelection { # [doc = " Use the standard pattern."] # [default] Standard = 0 , # [doc = " Use the standard_alpha_next_to_number pattern."] StandardAlphaNextToNumber = 1 , }
};
}
