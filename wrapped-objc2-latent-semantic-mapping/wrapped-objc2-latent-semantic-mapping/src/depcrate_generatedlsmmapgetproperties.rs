// Generated macro for LSMMapGetProperties (function)
macro_rules! Depcrate_generatedLSMMapGetProperties {
() => {
// Module: crate::generated
// Provides: {"LSMMapGetProperties"}
// Dependencies: {}
# [deprecated = "renamed to `LSMMap::properties`"] # [inline] pub unsafe extern "C-unwind" fn LSMMapGetProperties (mapref : & LSMMap) -> CFRetained < CFDictionary > { extern "C-unwind" { fn LSMMapGetProperties (mapref : & LSMMap) -> Option < NonNull < CFDictionary > > ; } let ret = unsafe { LSMMapGetProperties (mapref) } ; let ret = ret . expect ("function was marked as returning non-null, but actually returned NULL") ; unsafe { CFRetained :: retain (ret) } }
};
}
