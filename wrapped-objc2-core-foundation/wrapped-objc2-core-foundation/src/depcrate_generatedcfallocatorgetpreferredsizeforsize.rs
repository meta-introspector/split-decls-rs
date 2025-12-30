// Generated macro for CFAllocatorGetPreferredSizeForSize (function)
macro_rules! Depcrate_generatedCFAllocatorGetPreferredSizeForSize {
() => {
// Module: crate::generated
// Provides: {"CFAllocatorGetPreferredSizeForSize"}
// Dependencies: {}
# [deprecated = "renamed to `CFAllocator::preferred_size_for_size`"] # [inline] pub extern "C-unwind" fn CFAllocatorGetPreferredSizeForSize (allocator : Option < & CFAllocator > , size : CFIndex , hint : CFOptionFlags ,) -> CFIndex { extern "C-unwind" { fn CFAllocatorGetPreferredSizeForSize (allocator : Option < & CFAllocator > , size : CFIndex , hint : CFOptionFlags ,) -> CFIndex ; } unsafe { CFAllocatorGetPreferredSizeForSize (allocator , size , hint) } }
};
}
