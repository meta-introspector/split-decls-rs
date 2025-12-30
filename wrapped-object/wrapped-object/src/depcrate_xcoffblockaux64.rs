// Generated macro for BlockAux64 (struct)
macro_rules! Depcrate_xcoffBlockAux64 {
() => {
// Module: crate::xcoff
// Provides: {"BlockAux64"}
// Dependencies: {}
# [doc = " Block auxiliary entry for the C_BLOCK and C_FCN Symbols."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct BlockAux64 { # [doc = " Source line number."] pub x_lnno : U32 < BE > , # [doc = " Reserved."] pub pad : [u8 ; 13] , # [doc = " Contains _AUX_SYM; Type of auxiliary entry."] pub x_auxtype : u8 , }
};
}
