// Generated macro for SECURITY_ATTRIBUTES (struct)
macro_rules! Depcrate_bool_event_sans_referenceSECURITY_ATTRIBUTES {
() => {
// Module: crate::bool_event_sans_reference
// Provides: {"SECURITY_ATTRIBUTES"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy , Debug , PartialEq)] pub struct SECURITY_ATTRIBUTES { pub nLength : u32 , pub lpSecurityDescriptor : * mut core :: ffi :: c_void , pub bInheritHandle : windows_core :: BOOL , }
};
}
