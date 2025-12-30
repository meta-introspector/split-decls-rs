// Generated macro for one_literal (function)
macro_rules! Depcrate_literalone_literal {
() => {
// Module: crate::literal
// Provides: {"one_literal"}
// Dependencies: {}
fn one_literal (input : & [u8]) -> nom :: IResult < & [u8] , EvalResult , crate :: Error < & [u8] > > { alt ((map (full (c_char) , EvalResult :: Char) , map (full (c_int) , | i | EvalResult :: Int (:: std :: num :: Wrapping (i))) , map (full (c_float) , EvalResult :: Float) , map (full (c_string) , EvalResult :: Str) ,)) (input) . to_cexpr_result () }
};
}
