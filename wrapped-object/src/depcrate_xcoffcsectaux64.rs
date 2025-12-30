// Generated macro for CsectAux64 (struct)
macro_rules! Depcrate_xcoffCsectAux64 {
() => {
// Module: crate::xcoff
// Provides: {"CsectAux64"}
// Dependencies: {}
# [doc = " Csect auxiliary entry for C_EXT, C_WEAKEXT, and C_HIDEXT symbols."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct CsectAux64 { # [doc = " Low 4 bytes of section length."] pub x_scnlen_lo : U32 < BE > , # [doc = " Offset of parameter type-check hash in .typchk section."] pub x_parmhash : U32 < BE > , # [doc = " .typchk section number."] pub x_snhash : U16 < BE > , # [doc = " Symbol alignment and type."] pub x_smtyp : u8 , # [doc = " Storage mapping class."] pub x_smclas : u8 , # [doc = " High 4 bytes of section length."] pub x_scnlen_hi : U32 < BE > , # [doc = " Reserved."] pub pad : u8 , # [doc = " Contains _AUX_CSECT; indicates type of auxiliary entry."] pub x_auxtype : u8 , }
};
}
