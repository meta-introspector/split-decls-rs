// Generated macro for Exactness (enum)
macro_rules! Depcrate_units_providerExactness {
() => {
// Module: crate::units::provider
// Provides: {"Exactness"}
// Dependencies: {}
# [doc = " This enum is used to represent the exactness of a factor"] # [zerovec :: make_ule (ExactnessULE)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_experimental :: units :: provider))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [derive (Copy , Clone , Debug , PartialOrd , Ord , PartialEq , Eq , Default)] # [repr (u8)] pub enum Exactness { # [default] Exact = 0 , Approximate = 1 , }
};
}
