// Generated macro for impl_297 (impl)
macro_rules! Depcrate_stringimpl_297 {
() => {
// Module: crate::string
// Provides: {"impl_297"}
// Dependencies: {}
impl < 'a > From < & 'a CFString > for Cow < 'a , str > { fn from (cf_str : & 'a CFString) -> Cow < 'a , str > { unsafe { let c_string = CFStringGetCStringPtr (cf_str . 0 , kCFStringEncodingUTF8) ; if ! c_string . is_null () { let c_str = CStr :: from_ptr (c_string) ; Cow :: Borrowed (str :: from_utf8_unchecked (c_str . to_bytes ())) } else { let char_len = cf_str . char_len () ; let mut bytes_required : CFIndex = 0 ; CFStringGetBytes (cf_str . 0 , CFRange { location : 0 , length : char_len , } , kCFStringEncodingUTF8 , 0 , false as Boolean , ptr :: null_mut () , 0 , & mut bytes_required ,) ; let mut buffer = vec ! [b'\x00' ; bytes_required as usize] ; let mut bytes_used : CFIndex = 0 ; let chars_written = CFStringGetBytes (cf_str . 0 , CFRange { location : 0 , length : char_len , } , kCFStringEncodingUTF8 , 0 , false as Boolean , buffer . as_mut_ptr () , buffer . len () . to_CFIndex () , & mut bytes_used ,) ; assert_eq ! (chars_written , char_len) ; assert_eq ! (bytes_used , buffer . len () . to_CFIndex ()) ; Cow :: Owned (String :: from_utf8_unchecked (buffer)) } } } }
};
}
