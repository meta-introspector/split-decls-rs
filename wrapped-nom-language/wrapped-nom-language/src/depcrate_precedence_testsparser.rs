// Generated macro for parser (function)
macro_rules! Depcrate_precedence_testsparser {
() => {
// Module: crate::precedence::tests
// Provides: {"parser"}
// Dependencies: {}
fn parser (i : & str) -> IResult < & str , i64 > { precedence (unary_op (1 , tag ("-")) , fail () , alt ((binary_op (2 , Assoc :: Left , tag ("*")) , binary_op (2 , Assoc :: Left , tag ("/")) , binary_op (3 , Assoc :: Left , tag ("+")) , binary_op (3 , Assoc :: Left , tag ("-")) ,)) , alt ((map_res (digit1 , | s : & str | s . parse :: < i64 > ()) , delimited (tag ("(") , parser , tag (")")) ,)) , | op : Operation < & str , () , & str , i64 > | { use crate :: precedence :: Operation :: * ; match op { Prefix ("-" , o) => Ok (- o) , Binary (lhs , "*" , rhs) => Ok (lhs * rhs) , Binary (lhs , "/" , rhs) => Ok (lhs / rhs) , Binary (lhs , "+" , rhs) => Ok (lhs + rhs) , Binary (lhs , "-" , rhs) => Ok (lhs - rhs) , _ => Err ("Invalid combination") , } } ,) (i) }
};
}
