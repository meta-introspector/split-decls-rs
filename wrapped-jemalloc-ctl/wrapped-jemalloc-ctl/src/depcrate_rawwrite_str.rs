// Generated macro for write_str (function)
macro_rules! Depcrate_rawwrite_str {
() => {
// Module: crate::raw
// Provides: {"write_str"}
// Dependencies: {}
# [doc = " Uses the null-terminated string `name` as key to the _MALLCTL NAMESPACE_ and"] # [doc = " writes its `value`."] pub fn write_str (name : & [u8] , value : & 'static [u8]) -> Result < () > { assert ! (! value . is_empty () , "value cannot be empty") ; assert_eq ! (* value . last () . unwrap () , b'\0') ; unsafe { write (name , value . as_ptr () as * const c_char) } }
};
}
