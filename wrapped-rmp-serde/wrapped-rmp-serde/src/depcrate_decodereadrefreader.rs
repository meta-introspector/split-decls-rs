// Generated macro for ReadRefReader (struct)
macro_rules! Depcrate_decodeReadRefReader {
() => {
// Module: crate::decode
// Provides: {"ReadRefReader"}
// Dependencies: {}
# [doc = " Borrowed reader wrapper."] # [derive (Debug)] pub struct ReadRefReader < 'a , R : ? Sized > { whole_slice : & 'a R , buf : & 'a [u8] , }
};
}
