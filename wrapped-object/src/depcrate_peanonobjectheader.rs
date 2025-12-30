// Generated macro for AnonObjectHeader (struct)
macro_rules! Depcrate_peAnonObjectHeader {
() => {
// Module: crate::pe
// Provides: {"AnonObjectHeader"}
// Dependencies: {}
# [doc = " Non-COFF Object file header"] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct AnonObjectHeader { # [doc = " Must be IMAGE_FILE_MACHINE_UNKNOWN"] pub sig1 : U16 < LE > , # [doc = " Must be 0xffff"] pub sig2 : U16 < LE > , # [doc = " >= 1 (implies the ClsId field is present)"] pub version : U16 < LE > , pub machine : U16 < LE > , pub time_date_stamp : U32 < LE > , # [doc = " Used to invoke CoCreateInstance"] pub class_id : ClsId , # [doc = " Size of data that follows the header"] pub size_of_data : U32 < LE > , }
};
}
