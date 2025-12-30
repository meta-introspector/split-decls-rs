// Generated macro for gz_header (struct)
macro_rules! Depcrate_generatedgz_header {
() => {
// Module: crate::generated
// Provides: {"gz_header"}
// Dependencies: {}
# [repr (C)] # [derive (Copy , Clone)] pub struct gz_header { pub text : c_int , pub time : uLong , pub xflags : c_int , pub os : c_int , pub extra : * mut Bytef , pub extra_len : uInt , pub extra_max : uInt , pub name : * mut Bytef , pub name_max : uInt , pub comment : * mut Bytef , pub comm_max : uInt , pub hcrc : c_int , pub done : c_int , }
};
}
