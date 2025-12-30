// Generated macro for CFGetAllocator (function)
macro_rules! Depcrate_generatedCFGetAllocator {
() => {
// Module: crate::generated
// Provides: {"CFGetAllocator"}
// Dependencies: {}
# [inline] pub extern "C-unwind" fn CFGetAllocator (cf : Option < & CFType >) -> Option < CFRetained < CFAllocator > > { extern "C-unwind" { fn CFGetAllocator (cf : Option < & CFType >) -> Option < NonNull < CFAllocator > > ; } let ret = unsafe { CFGetAllocator (cf) } ; ret . map (| ret | unsafe { CFRetained :: retain (ret) }) }
};
}
