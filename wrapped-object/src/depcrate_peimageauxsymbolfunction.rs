// Generated macro for ImageAuxSymbolFunction (struct)
macro_rules! Depcrate_peImageAuxSymbolFunction {
() => {
// Module: crate::pe
// Provides: {"ImageAuxSymbolFunction"}
// Dependencies: {}
# [doc = " Auxiliary symbol format 1: function definitions."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageAuxSymbolFunction { pub tag_index : U32Bytes < LE > , pub total_size : U32Bytes < LE > , pub pointer_to_linenumber : U32Bytes < LE > , pub pointer_to_next_function : U32Bytes < LE > , pub unused : [u8 ; 2] , }
};
}
