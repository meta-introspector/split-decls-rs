// Generated macro for CFAllocatorReallocateCallBack (type)
macro_rules! Depcrate_baseCFAllocatorReallocateCallBack {
() => {
// Module: crate::base
// Provides: {"CFAllocatorReallocateCallBack"}
// Dependencies: {}
pub type CFAllocatorReallocateCallBack = extern "C" fn (ptr : * mut c_void , newsize : CFIndex , hint : CFOptionFlags , info : * mut c_void ,) -> * mut c_void ;
};
}
