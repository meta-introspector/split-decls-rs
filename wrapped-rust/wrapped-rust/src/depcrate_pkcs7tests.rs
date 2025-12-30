// Generated macro for tests (module)
macro_rules! Depcrate_pkcs7tests {
() => {
// Module: crate::pkcs7
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: borrow :: Cow ; use std :: ops :: Deref ; use super :: smime_canonicalize ; # [test] fn test_smime_canonicalize () { for (input , text_mode , expected_with_header , expected_without_header , expected_is_borrowed ,) in [(b"" as & [u8] , false , b"" as & [u8] , b"" as & [u8] , true) , (b"\n" , false , b"\r\n" , b"\r\n" , false) , (b"abc" , false , b"abc" , b"abc" , true) , (b"abc\r\ndef\n" , false , b"abc\r\ndef\r\n" , b"abc\r\ndef\r\n" , false ,) , (b"abc\r\n" , false , b"abc\r\n" , b"abc\r\n" , true) , (b"abc\ndef\n" , false , b"abc\r\ndef\r\n" , b"abc\r\ndef\r\n" , false ,) , (b"" , true , b"Content-Type: text/plain\r\n\r\n" , b"" , false) , (b"abc" , true , b"Content-Type: text/plain\r\n\r\nabc" , b"abc" , false ,) , (b"abc\n" , true , b"Content-Type: text/plain\r\n\r\nabc\r\n" , b"abc\r\n" , false ,) ,] { let (result_with_header , result_without_header) = smime_canonicalize (input , text_mode) ; assert_eq ! (result_with_header . deref () , expected_with_header) ; assert_eq ! (result_without_header . deref () , expected_without_header) ; assert_eq ! (matches ! (result_with_header , Cow :: Borrowed (_)) , expected_is_borrowed) ; assert_eq ! (matches ! (result_without_header , Cow :: Borrowed (_)) , expected_is_borrowed) ; } } }
};
}
