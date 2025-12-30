// Generated macro for LSMMapCreate (function)
macro_rules! Depcrate_generatedLSMMapCreate {
() => {
// Module: crate::generated
// Provides: {"LSMMapCreate"}
// Dependencies: {}
# [deprecated = "renamed to `LSMMap::new`"] # [inline] pub unsafe extern "C-unwind" fn LSMMapCreate (alloc : Option < & CFAllocator > , flags : CFOptionFlags ,) -> CFRetained < LSMMap > { extern "C-unwind" { fn LSMMapCreate (alloc : Option < & CFAllocator > , flags : CFOptionFlags ,) -> Option < NonNull < LSMMap > > ; } let ret = unsafe { LSMMapCreate (alloc , flags) } ; let ret = ret . expect ("function was marked as returning non-null, but actually returned NULL") ; unsafe { CFRetained :: from_raw (ret) } }
};
}
