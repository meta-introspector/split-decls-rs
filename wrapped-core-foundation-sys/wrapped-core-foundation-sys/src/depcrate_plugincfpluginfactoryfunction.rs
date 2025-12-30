// Generated macro for CFPlugInFactoryFunction (type)
macro_rules! Depcrate_pluginCFPlugInFactoryFunction {
() => {
// Module: crate::plugin
// Provides: {"CFPlugInFactoryFunction"}
// Dependencies: {}
pub type CFPlugInFactoryFunction = extern "C" fn (allocator : CFAllocatorRef , typeUUID : CFUUIDRef) -> * mut c_void ;
};
}
