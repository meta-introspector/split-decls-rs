// Generated macro for ImageSymbol (struct)
macro_rules! Depcrate_peImageSymbol {
() => {
// Module: crate::pe
// Provides: {"ImageSymbol"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageSymbol { # [doc = " If first 4 bytes are 0, then second 4 bytes are offset into string table."] pub name : [u8 ; 8] , pub value : U32Bytes < LE > , pub section_number : U16Bytes < LE > , pub typ : U16Bytes < LE > , pub storage_class : u8 , pub number_of_aux_symbols : u8 , }
};
}
