// Generated macro for ImageAlpha64RuntimeFunctionEntry (struct)
macro_rules! Depcrate_peImageAlpha64RuntimeFunctionEntry {
() => {
// Module: crate::pe
// Provides: {"ImageAlpha64RuntimeFunctionEntry"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageAlpha64RuntimeFunctionEntry { pub begin_address : U64 < LE > , pub end_address : U64 < LE > , pub exception_handler : U64 < LE > , pub handler_data : U64 < LE > , pub prolog_end_address : U64 < LE > , }
};
}
