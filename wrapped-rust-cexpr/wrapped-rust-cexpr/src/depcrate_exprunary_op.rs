// Generated macro for unary_op (function)
macro_rules! Depcrate_exprunary_op {
() => {
// Module: crate::expr
// Provides: {"unary_op"}
// Dependencies: {}
fn unary_op (input : (& [u8] , EvalResult)) -> Option < EvalResult > { use self :: EvalResult :: * ; assert_eq ! (input . 0 . len () , 1) ; match (input . 0 [0] , input . 1) { (b'+' , i) => Some (i) , (b'-' , Int (i)) => Some (Int (Wrapping (i . 0 . wrapping_neg ()))) , (b'-' , Float (i)) => Some (Float (- i)) , (b'-' , _) => unreachable ! ("non-numeric unary op") , (b'~' , Int (i)) => Some (Int (! i)) , (b'~' , Float (_)) => None , (b'~' , _) => unreachable ! ("non-numeric unary op") , _ => unreachable ! ("invalid unary op") , } }
};
}
