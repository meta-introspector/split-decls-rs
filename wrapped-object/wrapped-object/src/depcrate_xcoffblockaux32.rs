// Generated macro for BlockAux32 (struct)
macro_rules! Depcrate_xcoffBlockAux32 {
() => {
// Module: crate::xcoff
// Provides: {"BlockAux32"}
// Dependencies: {}
# [doc = " Block auxiliary entry for the C_BLOCK and C_FCN Symbols."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct BlockAux32 { # [doc = " Reserved."] pub pad : [u8 ; 2] , # [doc = " High-order 2 bytes of the source line number."] pub x_lnnohi : U16 < BE > , # [doc = " Low-order 2 bytes of the source line number."] pub x_lnnolo : U16 < BE > , # [doc = " Reserved."] pub pad2 : [u8 ; 12] , }
};
}
