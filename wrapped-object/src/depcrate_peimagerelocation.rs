// Generated macro for ImageRelocation (struct)
macro_rules! Depcrate_peImageRelocation {
() => {
// Module: crate::pe
// Provides: {"ImageRelocation"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageRelocation { # [doc = " Also `RelocCount` when IMAGE_SCN_LNK_NRELOC_OVFL is set"] pub virtual_address : U32Bytes < LE > , pub symbol_table_index : U32Bytes < LE > , pub typ : U16Bytes < LE > , }
};
}
