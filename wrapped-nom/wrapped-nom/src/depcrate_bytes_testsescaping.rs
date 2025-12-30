// Generated macro for escaping (function)
macro_rules! Depcrate_bytes_testsescaping {
() => {
// Module: crate::bytes::tests
// Provides: {"escaping"}
// Dependencies: {}
# [cfg (feature = "alloc")] # [allow (unused_variables)] # [test] fn escaping () { use crate :: character :: streaming :: one_of ; fn esc (i : & [u8]) -> IResult < & [u8] , & [u8] > { escaped (alpha , '\\' , one_of ("\"n\\")) (i) } assert_eq ! (esc (& b"abcd;" [..]) , Ok ((& b";" [..] , & b"abcd" [..]))) ; assert_eq ! (esc (& b"ab\\\"cd;" [..]) , Ok ((& b";" [..] , & b"ab\\\"cd" [..]))) ; assert_eq ! (esc (& b"\\\"abcd;" [..]) , Ok ((& b";" [..] , & b"\\\"abcd" [..]))) ; assert_eq ! (esc (& b"\\n;" [..]) , Ok ((& b";" [..] , & b"\\n" [..]))) ; assert_eq ! (esc (& b"ab\\\"12" [..]) , Ok ((& b"12" [..] , & b"ab\\\"" [..]))) ; assert_eq ! (esc (& b"AB\\" [..]) , Err (Err :: Error (error_position ! (& b"AB\\" [..] , ErrorKind :: Escaped)))) ; assert_eq ! (esc (& b"AB\\A" [..]) , Err (Err :: Error (error_node_position ! (& b"AB\\A" [..] , ErrorKind :: Escaped , error_position ! (& b"A" [..] , ErrorKind :: OneOf))))) ; fn esc2 (i : & [u8]) -> IResult < & [u8] , & [u8] > { escaped (digit , '\\' , one_of ("\"n\\")) (i) } assert_eq ! (esc2 (& b"12\\nnn34" [..]) , Ok ((& b"nn34" [..] , & b"12\\n" [..]))) ; }
};
}
