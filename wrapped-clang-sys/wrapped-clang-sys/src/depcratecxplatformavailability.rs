// Generated macro for CXPlatformAvailability (struct)
macro_rules! DepcrateCXPlatformAvailability {
() => {
// Module: crate
// Provides: {"CXPlatformAvailability"}
// Dependencies: {}
# [derive (Copy , Clone , Debug)] # [repr (C)] pub struct CXPlatformAvailability { pub Platform : CXString , pub Introduced : CXVersion , pub Deprecated : CXVersion , pub Obsoleted : CXVersion , pub Unavailable : c_int , pub Message : CXString , }
};
}
