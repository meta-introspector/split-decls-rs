// Generated macro for AixHeader (struct)
macro_rules! Depcrate_archiveAixHeader {
() => {
// Module: crate::archive
// Provides: {"AixHeader"}
// Dependencies: {}
# [doc = " The header at the start of an AIX big archive member, without name."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct AixHeader { # [doc = " File member size in decimal."] pub size : [u8 ; 20] , # [doc = " Next member offset in decimal."] pub nxtmem : [u8 ; 20] , # [doc = " Previous member offset in decimal."] pub prvmem : [u8 ; 20] , # [doc = " File member date in decimal."] pub date : [u8 ; 12] , # [doc = " File member user id in decimal."] pub uid : [u8 ; 12] , # [doc = " File member group id in decimal."] pub gid : [u8 ; 12] , # [doc = " File member mode in octal."] pub mode : [u8 ; 12] , # [doc = " File member name length in decimal."] pub namlen : [u8 ; 4] , }
};
}
