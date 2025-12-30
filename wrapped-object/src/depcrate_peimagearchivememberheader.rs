// Generated macro for ImageArchiveMemberHeader (struct)
macro_rules! Depcrate_peImageArchiveMemberHeader {
() => {
// Module: crate::pe
// Provides: {"ImageArchiveMemberHeader"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageArchiveMemberHeader { # [doc = " File member name - `/' terminated."] pub name : [u8 ; 16] , # [doc = " File member date - decimal."] pub date : [u8 ; 12] , # [doc = " File member user id - decimal."] pub user_id : [u8 ; 6] , # [doc = " File member group id - decimal."] pub group_id : [u8 ; 6] , # [doc = " File member mode - octal."] pub mode : [u8 ; 8] , # [doc = " File member size - decimal."] pub size : [u8 ; 10] , # [doc = " String to end header."] pub end_header : [u8 ; 2] , }
};
}
