// Generated macro for write_to_slice (function)
macro_rules! Depcrate_formatwrite_to_slice {
() => {
// Module: crate::format
// Provides: {"write_to_slice"}
// Dependencies: {}
# [doc = " Writes to the bytes buffer."] pub fn write_to_slice < 'a , T : fmt :: Display > (buf : & 'a mut [u8] , value : & T ,) -> Result < & 'a str , CapacityOverflowError > { let mut writer = ByteBufWriter { buffer : buf , cursor : 0 , } ; if write ! (writer , "{}" , value) . is_err () { return Err (CapacityOverflowError) ; } let len = writer . cursor ; let result = core :: str :: from_utf8 (& buf [.. len]) . expect ("[validity] fmt::Display writes valid UTF-8 byte sequence") ; Ok (result) }
};
}
