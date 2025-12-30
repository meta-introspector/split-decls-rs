// Generated macro for DefMapCrateData (struct)
macro_rules! Depcrate_nameresDefMapCrateData {
() => {
// Module: crate::nameres
// Provides: {"DefMapCrateData"}
// Dependencies: {}
# [doc = " Data that belongs to a crate which is shared between a crate's def map and all its block def maps."] # [derive (Clone , Debug , PartialEq , Eq)] struct DefMapCrateData { # [doc = " Side table for resolving derive helpers."] exported_derives : FxHashMap < MacroId , Box < [Name] > > , fn_proc_macro_mapping : FxHashMap < FunctionId , ProcMacroId > , # [doc = " Custom tool modules registered with `#![register_tool]`."] registered_tools : Vec < Symbol > , # [doc = " Unstable features of Rust enabled with `#![feature(A, B)]`."] unstable_features : FxHashSet < Symbol > , # [doc = " #[rustc_coherence_is_core]"] rustc_coherence_is_core : bool , no_core : bool , no_std : bool , edition : Edition , recursion_limit : Option < u32 > , }
};
}
