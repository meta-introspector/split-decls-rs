macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! Header {
    () => {
        deps!();
        # [doc = " The header at the start of an archive member."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Header { # [doc = " The file name."] pub name : [u8 ; 16] , # [doc = " File modification timestamp in decimal."] pub date : [u8 ; 12] , # [doc = " User ID in decimal."] pub uid : [u8 ; 6] , # [doc = " Group ID in decimal."] pub gid : [u8 ; 6] , # [doc = " File mode in octal."] pub mode : [u8 ; 8] , # [doc = " File size in decimal."] pub size : [u8 ; 10] , # [doc = " Must be equal to `TERMINATOR`."] pub terminator : [u8 ; 2] , }
    };
}

Header!();