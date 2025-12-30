// Generated macro for ExpAux (struct)
macro_rules! Depcrate_xcoffExpAux {
() => {
// Module: crate::xcoff
// Provides: {"ExpAux"}
// Dependencies: {}
# [doc = " Exception auxiliary entry. (XCOFF64 only)"] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ExpAux { # [doc = " File offset to exception table entry."] pub x_exptr : U64 < BE > , # [doc = " Size of function in bytes."] pub x_fsize : U32 < BE > , # [doc = " Symbol table index of next entry beyond this function."] pub x_endndx : U32 < BE > , # [doc = " Pad"] pub pad : u8 , # [doc = " Contains _AUX_EXCEPT; Type of auxiliary entry"] pub x_auxtype : u8 , }
};
}
