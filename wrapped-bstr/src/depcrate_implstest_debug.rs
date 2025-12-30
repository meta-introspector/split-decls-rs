// Generated macro for test_debug (function)
macro_rules! Depcrate_implstest_debug {
() => {
// Module: crate::impls
// Provides: {"test_debug"}
// Dependencies: {}
# [test] # [cfg (feature = "std")] fn test_debug () { use alloc :: format ; use crate :: { ByteSlice , B } ; assert_eq ! (r#""\0\0\0 ftypisom\0\0\x02\0isomiso2avc1mp""# , format ! ("{:?}" , b"\0\0\0 ftypisom\0\0\x02\0isomiso2avc1mp" . as_bstr ()) ,) ; assert_eq ! (b"\"\\xff\xef\xbf\xbd\\xff\"" . as_bstr () , B (& format ! ("{:?}" , b"\xff\xef\xbf\xbd\xff" . as_bstr ())) . as_bstr () ,) ; assert_eq ! (b"\"\\xed\\xa0\\x80Aa\\x7f\\x0b\"" . as_bstr () , B (& format ! ("{:?}" , b"\xed\xa0\x80Aa\x7f\x0b" . as_bstr ())) . as_bstr () ,) ; assert_eq ! (r#""\0\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x11\x12\r\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1f \x7f\x80\x81\xfe\xff""# , format ! ("{:?}" , b"\0\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x11\x12\r\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1f \x7f\x80\x81\xfe\xff" . as_bstr ()) ,) ; }
};
}
