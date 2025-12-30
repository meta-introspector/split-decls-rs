// Generated macro for CString (struct)
macro_rules! Depcrate_c_stringCString {
() => {
// Module: crate::c_string
// Provides: {"CString"}
// Dependencies: {}
# [doc = " A fixed capacity [`CString`](https://doc.rust-lang.org/std/ffi/struct.CString.html)."] # [doc = ""] # [doc = " It stores up to `N - 1` non-nul characters with a trailing nul terminator."] # [derive (Clone , Hash)] pub struct CString < const N : usize , LenT : LenType = usize > { inner : Vec < u8 , N , LenT > , }
};
}
