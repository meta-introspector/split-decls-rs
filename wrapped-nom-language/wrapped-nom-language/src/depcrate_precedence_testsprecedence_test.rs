// Generated macro for precedence_test (function)
macro_rules! Depcrate_precedence_testsprecedence_test {
() => {
// Module: crate::precedence::tests
// Provides: {"precedence_test"}
// Dependencies: {}
# [test] fn precedence_test () { assert_eq ! (parser ("3") , Ok (("" , 3))) ; assert_eq ! (parser ("-3") , Ok (("" , - 3))) ; assert_eq ! (parser ("4-(2*2)") , Ok (("" , 0))) ; assert_eq ! (parser ("4-2*2") , Ok (("" , 0))) ; assert_eq ! (parser ("(4-2)*2") , Ok (("" , 4))) ; assert_eq ! (parser ("2*2/1") , Ok (("" , 4))) ; let a = "a" ; assert_eq ! (parser (a) , Err (Err :: Error (error_node_position ! (& a [..] , ErrorKind :: Precedence , error_position ! (& a [..] , ErrorKind :: Tag))))) ; let b = "3+b" ; assert_eq ! (parser (b) , Err (Err :: Error (error_node_position ! (& b [2 ..] , ErrorKind :: Precedence , error_position ! (& b [2 ..] , ErrorKind :: Tag))))) ; }
};
}
