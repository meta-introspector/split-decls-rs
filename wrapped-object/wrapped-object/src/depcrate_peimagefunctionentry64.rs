// Generated macro for ImageFunctionEntry64 (struct)
macro_rules! Depcrate_peImageFunctionEntry64 {
() => {
// Module: crate::pe
// Provides: {"ImageFunctionEntry64"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageFunctionEntry64 { pub starting_address : U64 < LE > , pub ending_address : U64 < LE > , pub end_of_prologue_or_unwind_info_address : U64 < LE > , }
};
}
