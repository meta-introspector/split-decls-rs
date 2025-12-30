// Generated macro for LSMTextCreate (function)
macro_rules! Depcrate_generatedLSMTextCreate {
() => {
// Module: crate::generated
// Provides: {"LSMTextCreate"}
// Dependencies: {}
# [deprecated = "renamed to `LSMText::new`"] # [inline] pub unsafe extern "C-unwind" fn LSMTextCreate (alloc : Option < & CFAllocator > , mapref : & LSMMap ,) -> CFRetained < LSMText > { extern "C-unwind" { fn LSMTextCreate (alloc : Option < & CFAllocator > , mapref : & LSMMap) -> Option < NonNull < LSMText > > ; } let ret = unsafe { LSMTextCreate (alloc , mapref) } ; let ret = ret . expect ("function was marked as returning non-null, but actually returned NULL") ; unsafe { CFRetained :: from_raw (ret) } }
};
}
