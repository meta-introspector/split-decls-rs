// Generated macro for ImageRuntimeFunctionEntry (struct)
macro_rules! Depcrate_peImageRuntimeFunctionEntry {
() => {
// Module: crate::pe
// Provides: {"ImageRuntimeFunctionEntry"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageRuntimeFunctionEntry { pub begin_address : U32 < LE > , pub end_address : U32 < LE > , pub unwind_info_address_or_data : U32 < LE > , }
};
}
