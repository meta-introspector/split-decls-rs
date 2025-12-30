// Generated macro for take_while_m_n_utf8_full_match (function)
macro_rules! Depcrate_bytes_teststake_while_m_n_utf8_full_match {
() => {
// Module: crate::bytes::tests
// Provides: {"take_while_m_n_utf8_full_match"}
// Dependencies: {}
# [test] fn take_while_m_n_utf8_full_match () { use crate :: bytes :: streaming :: take_while_m_n ; fn parser (i : & str) -> IResult < & str , & str > { take_while_m_n (1 , 1 , | c : char | c . is_alphabetic ()) (i) } assert_eq ! (parser ("øn") , Ok (("n" , "ø"))) ; }
};
}
