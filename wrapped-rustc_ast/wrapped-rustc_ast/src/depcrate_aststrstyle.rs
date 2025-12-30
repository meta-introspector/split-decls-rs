// Generated macro for StrStyle (enum)
macro_rules! Depcrate_astStrStyle {
() => {
// Module: crate::ast
// Provides: {"StrStyle"}
// Dependencies: {}
# [derive (Clone , Encodable , Decodable , Debug , Copy , Hash , Eq , PartialEq)] # [derive (HashStable_Generic , Walkable)] pub enum StrStyle { # [doc = " A regular string, like `\"foo\"`."] Cooked , # [doc = " A raw string, like `r##\"foo\"##`."] # [doc = ""] # [doc = " The value is the number of `#` symbols used."] Raw (u8) , }
};
}
