// Generated macro for CFAllocatorGetDefault (function)
macro_rules! Depcrate_generatedCFAllocatorGetDefault {
() => {
// Module: crate::generated
// Provides: {"CFAllocatorGetDefault"}
// Dependencies: {}
# [deprecated = "renamed to `CFAllocator::default`"] # [inline] pub extern "C-unwind" fn CFAllocatorGetDefault () -> Option < CFRetained < CFAllocator > > { extern "C-unwind" { fn CFAllocatorGetDefault () -> Option < NonNull < CFAllocator > > ; } let ret = unsafe { CFAllocatorGetDefault () } ; ret . map (| ret | unsafe { CFRetained :: retain (ret) }) }
};
}
