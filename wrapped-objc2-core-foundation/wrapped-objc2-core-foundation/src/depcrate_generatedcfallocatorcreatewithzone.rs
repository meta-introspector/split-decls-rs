// Generated macro for CFAllocatorCreateWithZone (function)
macro_rules! Depcrate_generatedCFAllocatorCreateWithZone {
() => {
// Module: crate::generated
// Provides: {"CFAllocatorCreateWithZone"}
// Dependencies: {}
# [cfg (feature = "libc")] # [deprecated = "renamed to `CFAllocator::with_zone`"] # [inline] pub unsafe extern "C-unwind" fn CFAllocatorCreateWithZone (allocator : Option < & CFAllocator > , zone : * mut libc :: malloc_zone_t ,) -> Option < CFRetained < CFAllocator > > { extern "C-unwind" { fn CFAllocatorCreateWithZone (allocator : Option < & CFAllocator > , zone : * mut libc :: malloc_zone_t ,) -> Option < NonNull < CFAllocator > > ; } let ret = unsafe { CFAllocatorCreateWithZone (allocator , zone) } ; ret . map (| ret | unsafe { CFRetained :: from_raw (ret) }) }
};
}
