// Generated macro for SAFEARRAY (struct)
macro_rules! Depcrate_windows_sysSAFEARRAY {
() => {
// Module: crate::windows_sys
// Provides: {"SAFEARRAY"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy)] pub struct SAFEARRAY { pub cDims : u16 , pub fFeatures : ADVANCED_FEATURE_FLAGS , pub cbElements : u32 , pub cLocks : u32 , pub pvData : * mut core :: ffi :: c_void , pub rgsabound : [SAFEARRAYBOUND ; 1] , }
};
}
