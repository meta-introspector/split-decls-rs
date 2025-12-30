// Generated macro for ImageDebugMisc (struct)
macro_rules! Depcrate_peImageDebugMisc {
() => {
// Module: crate::pe
// Provides: {"ImageDebugMisc"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageDebugMisc { # [doc = " type of misc data, see defines"] pub data_type : U32 < LE > , # [doc = " total length of record, rounded to four byte multiple."] pub length : U32 < LE > , # [doc = " TRUE if data is unicode string"] pub unicode : u8 , pub reserved : [u8 ; 3] , }
};
}
