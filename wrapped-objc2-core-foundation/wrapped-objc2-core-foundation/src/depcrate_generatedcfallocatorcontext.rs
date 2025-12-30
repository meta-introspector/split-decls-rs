// Generated macro for CFAllocatorContext (struct)
macro_rules! Depcrate_generatedCFAllocatorContext {
() => {
// Module: crate::generated
// Provides: {"CFAllocatorContext"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfallocatorcontext?language=objc)"] # [repr (C)] # [allow (unpredictable_function_pointer_comparisons)] # [derive (Clone , Copy , Debug , PartialEq)] pub struct CFAllocatorContext { pub version : CFIndex , pub info : * mut c_void , pub retain : CFAllocatorRetainCallBack , pub release : CFAllocatorReleaseCallBack , pub copyDescription : CFAllocatorCopyDescriptionCallBack , pub allocate : CFAllocatorAllocateCallBack , pub reallocate : CFAllocatorReallocateCallBack , pub deallocate : CFAllocatorDeallocateCallBack , pub preferredSize : CFAllocatorPreferredSizeCallBack , }
};
}
