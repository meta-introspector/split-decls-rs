// Generated macro for impl_320 (impl)
macro_rules! Depcrate_arg_basic_implimpl_320 {
() => {
// Module: crate::arg::basic_impl
// Provides: {"impl_320"}
// Dependencies: {}
impl < 'a > Append for & 'a str { fn append_by_ref (& self , i : & mut IterAppend) { let b : & [u8] = self . as_bytes () ; let v : Cow < [u8] > = if ! b . is_empty () && b [b . len () - 1] == 0 { Cow :: Borrowed (b) } else { let mut bb : Vec < u8 > = b . into () ; bb . push (0) ; Cow :: Owned (bb) } ; let z = unsafe { CStr :: from_ptr (v . as_ptr () as * const c_char) } ; arg_append_str (& mut i . 0 , ArgType :: String , & z) } }
};
}
