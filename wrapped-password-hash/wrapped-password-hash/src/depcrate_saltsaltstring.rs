// Generated macro for SaltString (struct)
macro_rules! Depcrate_saltSaltString {
() => {
// Module: crate::salt
// Provides: {"SaltString"}
// Dependencies: {}
# [doc = " Owned stack-allocated equivalent of [`Salt`]."] # [derive (Clone , Eq)] pub struct SaltString { # [doc = " ASCII-encoded characters which comprise the salt."] chars : [u8 ; Salt :: MAX_LENGTH] , # [doc = " Length of the string in ASCII characters (i.e. bytes)."] length : u8 , }
};
}
