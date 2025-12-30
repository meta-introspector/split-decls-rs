// Generated macro for CFAllocatorCreate (function)
macro_rules! Depcrate_generatedCFAllocatorCreate {
() => {
// Module: crate::generated
// Provides: {"CFAllocatorCreate"}
// Dependencies: {}
# [deprecated = "renamed to `CFAllocator::new`"] # [inline] pub unsafe extern "C-unwind" fn CFAllocatorCreate (allocator : Option < & CFAllocator > , context : * mut CFAllocatorContext ,) -> Option < CFRetained < CFAllocator > > { extern "C-unwind" { fn CFAllocatorCreate (allocator : Option < & CFAllocator > , context : * mut CFAllocatorContext ,) -> Option < NonNull < CFAllocator > > ; } let ret = unsafe { CFAllocatorCreate (allocator , context) } ; ret . map (| ret | unsafe { CFRetained :: from_raw (ret) }) }
};
}
