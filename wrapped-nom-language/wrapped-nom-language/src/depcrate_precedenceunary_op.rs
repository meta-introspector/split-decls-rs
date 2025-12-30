// Generated macro for unary_op (function)
macro_rules! Depcrate_precedenceunary_op {
() => {
// Module: crate::precedence
// Provides: {"unary_op"}
// Dependencies: {}
# [doc = " Runs the inner parser and transforms the result into an unary operator with the given precedence."] # [doc = ""] # [doc = " Intended for use with [precedence]."] # [doc = " # Arguments"] # [doc = " * `precedence` The precedence of the operator."] # [doc = " * `parser` The parser to apply."] pub fn unary_op < I , O , E , P , Q > (precedence : Q , mut parser : P ,) -> impl FnMut (I) -> IResult < I , Unary < O , Q > , E > where P : Parser < I , Output = O , Error = E > , Q : Ord + Copy , { move | input | match parser . parse (input) { Ok ((i , value)) => Ok ((i , Unary { value , precedence })) , Err (e) => Err (e) , } }
};
}
