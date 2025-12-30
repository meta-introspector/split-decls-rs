// Generated macro for ImageResourceDataEntry (struct)
macro_rules! Depcrate_peImageResourceDataEntry {
() => {
// Module: crate::pe
// Provides: {"ImageResourceDataEntry"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageResourceDataEntry { # [doc = " RVA of the data."] pub offset_to_data : U32 < LE > , pub size : U32 < LE > , pub code_page : U32 < LE > , pub reserved : U32 < LE > , }
};
}
