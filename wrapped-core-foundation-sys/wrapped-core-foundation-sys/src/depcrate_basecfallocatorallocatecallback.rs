// Generated macro for CFAllocatorAllocateCallBack (type)
macro_rules! Depcrate_baseCFAllocatorAllocateCallBack {
() => {
// Module: crate::base
// Provides: {"CFAllocatorAllocateCallBack"}
// Dependencies: {}
pub type CFAllocatorAllocateCallBack = extern "C" fn (allocSize : CFIndex , hint : CFOptionFlags , info : * mut c_void) -> * mut c_void ;
};
}
