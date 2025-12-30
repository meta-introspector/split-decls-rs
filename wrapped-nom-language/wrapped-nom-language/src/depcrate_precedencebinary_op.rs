// Generated macro for binary_op (function)
macro_rules! Depcrate_precedencebinary_op {
() => {
// Module: crate::precedence
// Provides: {"binary_op"}
// Dependencies: {}
# [doc = " Runs the inner parser and transforms the result into a binary operator with the given precedence and associativity."] # [doc = ""] # [doc = " Intended for use with [precedence]."] # [doc = " # Arguments"] # [doc = " * `precedence` The precedence of the operator."] # [doc = " * `assoc` The associativity of the operator."] # [doc = " * `parser` The parser to apply."] pub fn binary_op < I , O , E , P , Q > (precedence : Q , assoc : Assoc , mut parser : P ,) -> impl FnMut (I) -> IResult < I , Binary < O , Q > , E > where P : Parser < I , Output = O , Error = E > , Q : Ord + Copy , { move | input | match parser . parse (input) { Ok ((i , value)) => Ok ((i , Binary { value , precedence , assoc , } ,)) , Err (e) => Err (e) , } }
};
}
