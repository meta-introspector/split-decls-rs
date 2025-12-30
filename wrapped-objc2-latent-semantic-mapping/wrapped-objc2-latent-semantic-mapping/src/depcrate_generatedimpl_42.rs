// Generated macro for impl_42 (impl)
macro_rules! Depcrate_generatedimpl_42 {
() => {
// Module: crate::generated
// Provides: {"impl_42"}
// Dependencies: {}
impl LSMResult { # [doc = " Returns, in decreasing order of likelihood, the categories or words"] # [doc = " that best match when a text is mapped into a map."] # [doc (alias = "LSMResultCreate")] # [inline] pub unsafe fn new (alloc : Option < & CFAllocator > , mapref : & LSMMap , textref : & LSMText , num_results : CFIndex , flags : CFOptionFlags ,) -> CFRetained < LSMResult > { extern "C-unwind" { fn LSMResultCreate (alloc : Option < & CFAllocator > , mapref : & LSMMap , textref : & LSMText , num_results : CFIndex , flags : CFOptionFlags ,) -> Option < NonNull < LSMResult > > ; } let ret = unsafe { LSMResultCreate (alloc , mapref , textref , num_results , flags) } ; let ret = ret . expect ("function was marked as returning non-null, but actually returned NULL") ; unsafe { CFRetained :: from_raw (ret) } } }
};
}
