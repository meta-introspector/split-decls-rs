// Generated macro for ScriptExtensionsSet (struct)
macro_rules! Depcrate_scriptScriptExtensionsSet {
() => {
// Module: crate::script
// Provides: {"ScriptExtensionsSet"}
// Dependencies: {}
# [doc = " A struct that wraps a [`Script`] array, such as in the return value for"] # [doc = " [`get_script_extensions_val()`](ScriptWithExtensionsBorrowed::get_script_extensions_val)."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct ScriptExtensionsSet < 'a > { values : & 'a ZeroSlice < Script > , }
};
}
