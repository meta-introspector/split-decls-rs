// Generated macro for CFAllocatorAllocateBytes (function)
macro_rules! Depcrate_generatedCFAllocatorAllocateBytes {
() => {
// Module: crate::generated
// Provides: {"CFAllocatorAllocateBytes"}
// Dependencies: {}
# [deprecated = "renamed to `CFAllocator::allocate_bytes`"] # [inline] pub extern "C-unwind" fn CFAllocatorAllocateBytes (allocator : Option < & CFAllocator > , size : CFIndex , hint : CFOptionFlags ,) -> * mut c_void { extern "C-unwind" { fn CFAllocatorAllocateBytes (allocator : Option < & CFAllocator > , size : CFIndex , hint : CFOptionFlags ,) -> * mut c_void ; } unsafe { CFAllocatorAllocateBytes (allocator , size , hint) } }
};
}
