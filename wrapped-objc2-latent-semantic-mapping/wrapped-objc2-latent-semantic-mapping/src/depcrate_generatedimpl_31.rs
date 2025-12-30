// Generated macro for impl_31 (impl)
macro_rules! Depcrate_generatedimpl_31 {
() => {
// Module: crate::generated
// Provides: {"impl_31"}
// Dependencies: {}
impl LSMMap { # [doc = " Creates a new LSM map. Call CFRelease to dispose."] # [doc (alias = "LSMMapCreate")] # [inline] pub unsafe fn new (alloc : Option < & CFAllocator > , flags : CFOptionFlags) -> CFRetained < LSMMap > { extern "C-unwind" { fn LSMMapCreate (alloc : Option < & CFAllocator > , flags : CFOptionFlags ,) -> Option < NonNull < LSMMap > > ; } let ret = unsafe { LSMMapCreate (alloc , flags) } ; let ret = ret . expect ("function was marked as returning non-null, but actually returned NULL") ; unsafe { CFRetained :: from_raw (ret) } } }
};
}
