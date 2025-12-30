// Generated macro for CXStringSet (struct)
macro_rules! DepcrateCXStringSet {
() => {
// Module: crate
// Provides: {"CXStringSet"}
// Dependencies: {}
# [cfg (feature = "clang_3_8")] # [derive (Copy , Clone , Debug)] # [repr (C)] pub struct CXStringSet { pub Strings : * mut CXString , pub Count : c_uint , }
};
}
