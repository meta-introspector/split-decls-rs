// Generated macro for single_element_tuples (function)
macro_rules! Depcrate_sequence_testssingle_element_tuples {
() => {
// Module: crate::sequence::tests
// Provides: {"single_element_tuples"}
// Dependencies: {}
# [test] fn single_element_tuples () { use crate :: character :: complete :: alpha1 ; use crate :: { error :: ErrorKind , Err } ; let mut parser = (alpha1 ,) ; assert_eq ! (crate :: Parser :: parse (& mut parser , "abc123def") , Ok (("123def" , ("abc" ,)))) ; assert_eq ! (crate :: Parser :: parse (& mut parser , "123def") , Err (Err :: Error (("123def" , ErrorKind :: Alpha)))) ; }
};
}
