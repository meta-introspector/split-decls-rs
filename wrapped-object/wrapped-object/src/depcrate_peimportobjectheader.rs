// Generated macro for ImportObjectHeader (struct)
macro_rules! Depcrate_peImportObjectHeader {
() => {
// Module: crate::pe
// Provides: {"ImportObjectHeader"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImportObjectHeader { # [doc = " Must be IMAGE_FILE_MACHINE_UNKNOWN"] pub sig1 : U16 < LE > , # [doc = " Must be IMPORT_OBJECT_HDR_SIG2."] pub sig2 : U16 < LE > , pub version : U16 < LE > , pub machine : U16 < LE > , # [doc = " Time/date stamp"] pub time_date_stamp : U32 < LE > , # [doc = " particularly useful for incremental links"] pub size_of_data : U32 < LE > , # [doc = " if grf & IMPORT_OBJECT_ORDINAL"] pub ordinal_or_hint : U16 < LE > , pub name_type : U16 < LE > , }
};
}
