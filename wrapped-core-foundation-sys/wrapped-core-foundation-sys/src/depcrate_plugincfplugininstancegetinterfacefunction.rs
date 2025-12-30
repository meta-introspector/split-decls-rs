// Generated macro for CFPlugInInstanceGetInterfaceFunction (type)
macro_rules! Depcrate_pluginCFPlugInInstanceGetInterfaceFunction {
() => {
// Module: crate::plugin
// Provides: {"CFPlugInInstanceGetInterfaceFunction"}
// Dependencies: {}
pub type CFPlugInInstanceGetInterfaceFunction = extern "C" fn (instance : CFPlugInInstanceRef , interfaceName : CFStringRef , ftbl : * mut * mut c_void ,) -> Boolean ;
};
}
