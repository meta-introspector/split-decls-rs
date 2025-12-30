// Generated macro for CFAllocatorSetDefault (function)
macro_rules! Depcrate_generatedCFAllocatorSetDefault {
() => {
// Module: crate::generated
// Provides: {"CFAllocatorSetDefault"}
// Dependencies: {}
# [deprecated = "renamed to `CFAllocator::set_default`"] # [inline] pub extern "C-unwind" fn CFAllocatorSetDefault (allocator : Option < & CFAllocator >) { extern "C-unwind" { fn CFAllocatorSetDefault (allocator : Option < & CFAllocator >) ; } unsafe { CFAllocatorSetDefault (allocator) } }
};
}
