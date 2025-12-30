// Generated macro for format_macl (function)
macro_rules! Depcrate_fs_feature_xattrformat_macl {
() => {
// Module: crate::fs::feature::xattr
// Provides: {"format_macl"}
// Dependencies: {}
# [cfg (target_os = "macos")] fn format_macl (value : & [u8]) -> String { const HEX : [u8 ; 16] = [b'0' , b'1' , b'2' , b'3' , b'4' , b'5' , b'6' , b'7' , b'8' , b'9' , b'a' , b'b' , b'c' , b'd' , b'e' , b'f' ,] ; const GROUPS : [(usize , usize , u8) ; 6] = [(0 , 4 , b';') , (5 , 13 , b'-') , (14 , 18 , b'-') , (19 , 23 , b'-') , (24 , 28 , b'-') , (29 , 41 , 0) ,] ; let mut dst = [0 ; 41] ; let mut i = 0 ; for (start , end , sep) in GROUPS { for j in (start .. end) . step_by (2) { let x = value [i] ; i += 1 ; dst [j] = HEX [(x >> 4) as usize] ; dst [j + 1] = HEX [(x & 0x0f) as usize] ; } if sep != 0 { dst [end] = sep ; } } unsafe { String :: from_utf8_unchecked (dst . to_vec ()) } }
};
}
