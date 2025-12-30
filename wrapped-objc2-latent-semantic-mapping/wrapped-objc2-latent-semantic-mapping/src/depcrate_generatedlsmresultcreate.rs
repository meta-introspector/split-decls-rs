// Generated macro for LSMResultCreate (function)
macro_rules! Depcrate_generatedLSMResultCreate {
() => {
// Module: crate::generated
// Provides: {"LSMResultCreate"}
// Dependencies: {}
# [deprecated = "renamed to `LSMResult::new`"] # [inline] pub unsafe extern "C-unwind" fn LSMResultCreate (alloc : Option < & CFAllocator > , mapref : & LSMMap , textref : & LSMText , num_results : CFIndex , flags : CFOptionFlags ,) -> CFRetained < LSMResult > { extern "C-unwind" { fn LSMResultCreate (alloc : Option < & CFAllocator > , mapref : & LSMMap , textref : & LSMText , num_results : CFIndex , flags : CFOptionFlags ,) -> Option < NonNull < LSMResult > > ; } let ret = unsafe { LSMResultCreate (alloc , mapref , textref , num_results , flags) } ; let ret = ret . expect ("function was marked as returning non-null, but actually returned NULL") ; unsafe { CFRetained :: from_raw (ret) } }
};
}
