// Generated macro for escaped_braces (function)
macro_rules! Depcrate_testsescaped_braces {
() => {
// Module: crate::tests
// Provides: {"escaped_braces"}
// Dependencies: {}
# [rstest] # [case ("}}" , "}")] # [case ("{{" , "{")] # [case ("literal{{literal" , "literal{literal")] # [case ("literal}}literal" , "literal}literal")] # [case ("{{}}" , "{}")] # [case ("}}{{" , "}{")] fn escaped_braces (# [case] input : & str , # [case] literal : & str) { assert_eq ! (parse (input , ParserMode :: Strict) , Ok (vec ! [Fragment :: Literal (literal . into ())])) ; }
};
}
