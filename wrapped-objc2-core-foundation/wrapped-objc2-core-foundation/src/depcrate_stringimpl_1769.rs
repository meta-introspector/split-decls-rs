// Generated macro for impl_1769 (impl)
macro_rules! Depcrate_stringimpl_1769 {
() => {
// Module: crate::string
// Provides: {"impl_1769"}
// Dependencies: {}
impl fmt :: Display for CFString { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut buf = [0u8 ; 32] ; let mut location_utf16 = 0 ; loop { let len_utf16 = self . length () ; let mut read_utf8 = 0 ; let read_utf16 = unsafe { self . bytes (CFRange { location : location_utf16 , length : len_utf16 - location_utf16 , } , CFStringBuiltInEncodings :: EncodingUTF8 . 0 , 0 , false , buf . as_mut_ptr () , buf . len () as _ , & mut read_utf8 ,) } ; if read_utf16 <= 0 { if location_utf16 < len_utf16 { f . write_char (char :: REPLACEMENT_CHARACTER) ? ; location_utf16 += 1 ; continue ; } break ; } location_utf16 += read_utf16 ; let s = unsafe { debug_checked_utf8_unchecked (& buf [0 .. read_utf8 as usize]) } ; f . write_str (s) ? ; } Ok (()) } }
};
}
