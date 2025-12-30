// Generated macro for CFAllocatorReallocateCallBack (type)
macro_rules! Depcrate_generatedCFAllocatorReallocateCallBack {
() => {
// Module: crate::generated
// Provides: {"CFAllocatorReallocateCallBack"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfallocatorreallocatecallback?language=objc)"] pub type CFAllocatorReallocateCallBack = Option < unsafe extern "C-unwind" fn (* mut c_void , CFIndex , CFOptionFlags , * mut c_void) -> * mut c_void , > ;
};
}
