// Generated macro for Inflation (struct)
macro_rules! DepcrateInflation {
() => {
// Module: crate
// Provides: {"Inflation"}
// Dependencies: {}
# [cfg_attr (feature = "frozen-abi" , derive (solana_frozen_abi_macro :: AbiExample))] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (PartialEq , Clone , Debug , Copy)] # [cfg_attr (feature = "serde" , serde (rename_all = "camelCase"))] pub struct Inflation { # [doc = " Initial inflation percentage, from time=0"] pub initial : f64 , # [doc = " Terminal inflation percentage, to time=INF"] pub terminal : f64 , # [doc = " Rate per year, at which inflation is lowered until reaching terminal"] # [doc = "  i.e. inflation(year) == MAX(terminal, initial*((1-taper)^year))"] pub taper : f64 , # [doc = " Percentage of total inflation allocated to the foundation"] pub foundation : f64 , # [doc = " Duration of foundation pool inflation, in years"] pub foundation_term : f64 , # [doc = " DEPRECATED, this field is currently unused"] __unused : f64 , }
};
}
