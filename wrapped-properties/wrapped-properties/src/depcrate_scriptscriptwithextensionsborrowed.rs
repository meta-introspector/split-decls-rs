// Generated macro for ScriptWithExtensionsBorrowed (struct)
macro_rules! Depcrate_scriptScriptWithExtensionsBorrowed {
() => {
// Module: crate::script
// Provides: {"ScriptWithExtensionsBorrowed"}
// Dependencies: {}
# [doc = " A borrowed wrapper around script extension data, returned by"] # [doc = " [`ScriptWithExtensions::as_borrowed()`]. More efficient to query."] # [derive (Clone , Copy , Debug)] pub struct ScriptWithExtensionsBorrowed < 'a > { data : & 'a ScriptWithExtensionsProperty < 'a > , }
};
}
