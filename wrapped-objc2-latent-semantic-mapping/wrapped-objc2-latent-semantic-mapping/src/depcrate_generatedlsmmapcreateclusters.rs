// Generated macro for LSMMapCreateClusters (function)
macro_rules! Depcrate_generatedLSMMapCreateClusters {
() => {
// Module: crate::generated
// Provides: {"LSMMapCreateClusters"}
// Dependencies: {}
# [deprecated = "renamed to `LSMMap::new_clusters`"] # [inline] pub unsafe extern "C-unwind" fn LSMMapCreateClusters (alloc : Option < & CFAllocator > , mapref : & LSMMap , subset : Option < & CFArray > , num_clusters : CFIndex , flags : CFOptionFlags ,) -> Option < CFRetained < CFArray > > { extern "C-unwind" { fn LSMMapCreateClusters (alloc : Option < & CFAllocator > , mapref : & LSMMap , subset : Option < & CFArray > , num_clusters : CFIndex , flags : CFOptionFlags ,) -> Option < NonNull < CFArray > > ; } let ret = unsafe { LSMMapCreateClusters (alloc , mapref , subset , num_clusters , flags) } ; ret . map (| ret | unsafe { CFRetained :: from_raw (ret) }) }
};
}
