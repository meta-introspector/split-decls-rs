// Generated macro for ImageAuxSymbolFunctionBeginEnd (struct)
macro_rules! Depcrate_peImageAuxSymbolFunctionBeginEnd {
() => {
// Module: crate::pe
// Provides: {"ImageAuxSymbolFunctionBeginEnd"}
// Dependencies: {}
# [doc = " Auxiliary symbol format 2: .bf and .ef symbols."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageAuxSymbolFunctionBeginEnd { pub unused1 : [u8 ; 4] , # [doc = " declaration line number"] pub linenumber : U16Bytes < LE > , pub unused2 : [u8 ; 6] , pub pointer_to_next_function : U32Bytes < LE > , pub unused3 : [u8 ; 2] , }
};
}
