// Generated macro for FunAux64 (struct)
macro_rules! Depcrate_xcoffFunAux64 {
() => {
// Module: crate::xcoff
// Provides: {"FunAux64"}
// Dependencies: {}
# [doc = " Function auxiliary entry."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct FunAux64 { # [doc = " File pointer to line number"] pub x_lnnoptr : U64 < BE > , # [doc = " Size of function in bytes."] pub x_fsize : U32 < BE > , # [doc = " Symbol table index of next entry beyond this function."] pub x_endndx : U32 < BE > , # [doc = " Pad"] pub pad : u8 , # [doc = " Contains _AUX_FCN; Type of auxiliary entry."] pub x_auxtype : u8 , }
};
}
