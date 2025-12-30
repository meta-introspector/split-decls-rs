// Generated macro for CFAllocatorAllocate (function)
macro_rules! Depcrate_generatedCFAllocatorAllocate {
() => {
// Module: crate::generated
// Provides: {"CFAllocatorAllocate"}
// Dependencies: {}
# [deprecated = "renamed to `CFAllocator::allocate`"] # [inline] pub extern "C-unwind" fn CFAllocatorAllocate (allocator : Option < & CFAllocator > , size : CFIndex , hint : CFOptionFlags ,) -> * mut c_void { extern "C-unwind" { fn CFAllocatorAllocate (allocator : Option < & CFAllocator > , size : CFIndex , hint : CFOptionFlags ,) -> * mut c_void ; } unsafe { CFAllocatorAllocate (allocator , size , hint) } }
};
}
