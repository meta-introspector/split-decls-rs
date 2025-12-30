// Generated macro for impl_41 (impl)
macro_rules! Depcrate_generatedimpl_41 {
() => {
// Module: crate::generated
// Provides: {"impl_41"}
// Dependencies: {}
impl LSMMap { # [doc = " Group categories or words (tokens) into the specified sets of clusters."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `clusters` generic must be of the correct type."] # [doc (alias = "LSMMapApplyClusters")] # [inline] pub unsafe fn apply_clusters (& self , clusters : & CFArray) -> OSStatus { extern "C-unwind" { fn LSMMapApplyClusters (mapref : & LSMMap , clusters : & CFArray) -> OSStatus ; } unsafe { LSMMapApplyClusters (self , clusters) } } }
};
}
