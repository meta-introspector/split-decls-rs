// Generated macro for LSMMapCreateFromURL (function)
macro_rules! Depcrate_generatedLSMMapCreateFromURL {
() => {
// Module: crate::generated
// Provides: {"LSMMapCreateFromURL"}
// Dependencies: {}
# [deprecated = "renamed to `LSMMap::from_url`"] # [inline] pub unsafe extern "C-unwind" fn LSMMapCreateFromURL (alloc : Option < & CFAllocator > , file : & CFURL , flags : CFOptionFlags ,) -> Option < CFRetained < LSMMap > > { extern "C-unwind" { fn LSMMapCreateFromURL (alloc : Option < & CFAllocator > , file : & CFURL , flags : CFOptionFlags ,) -> Option < NonNull < LSMMap > > ; } let ret = unsafe { LSMMapCreateFromURL (alloc , file , flags) } ; ret . map (| ret | unsafe { CFRetained :: from_raw (ret) }) }
};
}
