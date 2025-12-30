// Generated macro for ThinShared (struct)
macro_rules! Depcrate_back_ltoThinShared {
() => {
// Module: crate::back::lto
// Provides: {"ThinShared"}
// Dependencies: {}
pub struct ThinShared < B : WriteBackendMethods > { pub data : B :: ThinData , pub thin_buffers : Vec < B :: ThinBuffer > , pub serialized_modules : Vec < SerializedModule < B :: ModuleBuffer > > , pub module_names : Vec < CString > , }
};
}
