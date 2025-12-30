// Generated macro for CXIndexOptions (struct)
macro_rules! DepcrateCXIndexOptions {
() => {
// Module: crate
// Provides: {"CXIndexOptions"}
// Dependencies: {}
# [cfg (feature = "clang_17_0")] # [derive (Copy , Clone , Debug)] # [repr (C)] pub struct CXIndexOptions { pub Size : c_uint , pub ThreadBackgroundPriorityForIndexing : CXChoice , pub ThreadBackgroundPriorityForEditing : CXChoice , pub flags : CXIndexOptions_Flags , pub PreambleStoragePath : * const c_char , pub InvocationEmissionPath : * const c_char , }
};
}
