// Generated macro for issue_1027_convert_error_panic_nonempty (function)
macro_rules! Depcrate_errorissue_1027_convert_error_panic_nonempty {
() => {
// Module: crate::error
// Provides: {"issue_1027_convert_error_panic_nonempty"}
// Dependencies: {}
# [test] fn issue_1027_convert_error_panic_nonempty () { use nom :: character :: complete :: char ; use nom :: sequence :: pair ; use nom :: Err ; use nom :: IResult ; use nom :: Parser ; let input = "a" ; let result : IResult < _ , _ , VerboseError < & str > > = pair (char ('a') , char ('b')) . parse (input) ; let err = match result . unwrap_err () { Err :: Error (e) => e , _ => unreachable ! () , } ; let msg = convert_error (input , err) ; assert_eq ! (msg , "0: at line 1:\na\n ^\nexpected \'b\', got end of input\n\n") ; }
};
}
