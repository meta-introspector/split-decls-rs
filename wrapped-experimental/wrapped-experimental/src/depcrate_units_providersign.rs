// Generated macro for Sign (enum)
macro_rules! Depcrate_units_providerSign {
() => {
// Module: crate::units::provider
// Provides: {"Sign"}
// Dependencies: {}
# [doc = " This enum is used to represent the sign of a constant value."] # [zerovec :: make_ule (SignULE)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_experimental :: units :: provider))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [derive (Copy , Clone , Debug , PartialOrd , Ord , PartialEq , Eq , Default)] # [repr (u8)] pub enum Sign { # [default] Positive = 0 , Negative = 1 , }
};
}
