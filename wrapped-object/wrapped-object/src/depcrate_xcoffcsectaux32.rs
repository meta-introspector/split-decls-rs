// Generated macro for CsectAux32 (struct)
macro_rules! Depcrate_xcoffCsectAux32 {
() => {
// Module: crate::xcoff
// Provides: {"CsectAux32"}
// Dependencies: {}
# [doc = " Csect auxiliary entry for C_EXT, C_WEAKEXT, and C_HIDEXT symbols."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct CsectAux32 { # [doc = " Section length."] pub x_scnlen : U32 < BE > , # [doc = " Offset of parameter type-check hash in .typchk section."] pub x_parmhash : U32 < BE > , # [doc = " .typchk section number."] pub x_snhash : U16 < BE > , # [doc = " Symbol alignment and type."] pub x_smtyp : u8 , # [doc = " Storage mapping class."] pub x_smclas : u8 , # [doc = " Reserved."] pub x_stab : U32 < BE > , # [doc = " x_snstab."] pub x_snstab : U16 < BE > , }
};
}
