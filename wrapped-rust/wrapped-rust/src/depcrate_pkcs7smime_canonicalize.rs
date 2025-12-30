// Generated macro for smime_canonicalize (function)
macro_rules! Depcrate_pkcs7smime_canonicalize {
() => {
// Module: crate::pkcs7
// Provides: {"smime_canonicalize"}
// Dependencies: {}
fn smime_canonicalize (data : & [u8] , text_mode : bool) -> (Cow < '_ , [u8] > , Cow < '_ , [u8] >) { let mut new_data_with_header = vec ! [] ; let mut new_data_without_header = vec ! [] ; if text_mode { new_data_with_header . extend_from_slice (b"Content-Type: text/plain\r\n\r\n") ; } let mut last_idx = 0 ; for (i , c) in data . iter () . copied () . enumerate () { if c == b'\n' && (i == 0 || data [i - 1] != b'\r') { new_data_with_header . extend_from_slice (& data [last_idx .. i]) ; new_data_with_header . push (b'\r') ; new_data_with_header . push (b'\n') ; new_data_without_header . extend_from_slice (& data [last_idx .. i]) ; new_data_without_header . push (b'\r') ; new_data_without_header . push (b'\n') ; last_idx = i + 1 ; } } if ! new_data_with_header . is_empty () { new_data_with_header . extend_from_slice (& data [last_idx ..]) ; new_data_without_header . extend_from_slice (& data [last_idx ..]) ; (Cow :: Owned (new_data_with_header) , Cow :: Owned (new_data_without_header) ,) } else { (Cow :: Borrowed (data) , Cow :: Borrowed (data)) } }
};
}
