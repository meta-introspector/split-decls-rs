// Generated macro for ImageAlphaRuntimeFunctionEntry (struct)
macro_rules! Depcrate_peImageAlphaRuntimeFunctionEntry {
() => {
// Module: crate::pe
// Provides: {"ImageAlphaRuntimeFunctionEntry"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageAlphaRuntimeFunctionEntry { pub begin_address : U32 < LE > , pub end_address : U32 < LE > , pub exception_handler : U32 < LE > , pub handler_data : U32 < LE > , pub prolog_end_address : U32 < LE > , }
};
}
