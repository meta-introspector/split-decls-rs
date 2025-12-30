// Generated macro for CFAllocatorContext (struct)
macro_rules! Depcrate_baseCFAllocatorContext {
() => {
// Module: crate::base
// Provides: {"CFAllocatorContext"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy , Debug)] pub struct CFAllocatorContext { pub version : CFIndex , pub info : * mut c_void , pub retain : Option < CFAllocatorRetainCallBack > , pub release : Option < CFAllocatorReleaseCallBack > , pub copyDescription : Option < CFAllocatorCopyDescriptionCallBack > , pub allocate : Option < CFAllocatorAllocateCallBack > , pub reallocate : Option < CFAllocatorReallocateCallBack > , pub deallocate : Option < CFAllocatorDeallocateCallBack > , pub preferredSize : Option < CFAllocatorPreferredSizeCallBack > , }
};
}
