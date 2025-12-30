// Generated macro for Punctuated (struct)
macro_rules! DepcratePunctuated {
() => {
// Module: crate
// Provides: {"Punctuated"}
// Dependencies: {}
# [doc = " Type of a punctuated list."] # [doc = ""] # [doc = " This refers to `syn::punctuated::Punctuated<#element, #punct>`."] # [doc = ""] # [doc = " The punct string will match one of the keys in the `tokens` map."] # [derive (Clone , Debug , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Punctuated { pub element : Box < Type > , pub punct : String , }
};
}
