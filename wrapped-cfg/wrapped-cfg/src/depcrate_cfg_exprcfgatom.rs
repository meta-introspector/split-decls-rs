// Generated macro for CfgAtom (enum)
macro_rules! Depcrate_cfg_exprCfgAtom {
() => {
// Module: crate::cfg_expr
// Provides: {"CfgAtom"}
// Dependencies: {}
# [doc = " A simple configuration value passed in from the outside."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub enum CfgAtom { # [doc = " eg. `#[cfg(test)]`"] Flag (Symbol) , # [doc = " eg. `#[cfg(target_os = \"linux\")]`"] # [doc = ""] # [doc = " Note that a key can have multiple values that are all considered \"active\" at the same time."] # [doc = " For example, `#[cfg(target_feature = \"sse\")]` and `#[cfg(target_feature = \"sse2\")]`."] KeyValue { key : Symbol , value : Symbol } , }
};
}
