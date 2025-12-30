// Generated macro for ImageAuxSymbolTokenDef (struct)
macro_rules! Depcrate_peImageAuxSymbolTokenDef {
() => {
// Module: crate::pe
// Provides: {"ImageAuxSymbolTokenDef"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageAuxSymbolTokenDef { # [doc = " IMAGE_AUX_SYMBOL_TYPE"] pub aux_type : u8 , # [doc = " Must be 0"] pub reserved1 : u8 , pub symbol_table_index : U32Bytes < LE > , # [doc = " Must be 0"] pub reserved2 : [u8 ; 12] , }
};
}
