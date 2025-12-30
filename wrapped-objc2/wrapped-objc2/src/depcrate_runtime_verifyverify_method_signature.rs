// Generated macro for verify_method_signature (function)
macro_rules! Depcrate_runtime_verifyverify_method_signature {
() => {
// Module: crate::runtime::verify
// Provides: {"verify_method_signature"}
// Dependencies: {}
pub (crate) fn verify_method_signature (method : & Method , args : & [Encoding] , ret : & Encoding ,) -> Result < () , VerificationError > { let mut iter = method . types () ; if * ret != Encoding :: None { let (expected , _stack_layout) = iter . extract_return () ? ; if ! relaxed_equivalent_to_box (ret , & expected) { return Err (Inner :: MismatchedReturn (expected , ret . clone ()) . into ()) ; } } iter . verify_receiver () ? ; iter . verify_sel () ? ; let actual_count = args . len () ; for (i , actual) in args . iter () . enumerate () { if * actual == Encoding :: None { continue ; } if let Some (res) = iter . next () { let (expected , _stack_layout) = res ? ; let mut equivalent = relaxed_equivalent_to_box (actual , & expected) ; if ! equivalent { if let Encoding :: Array (_size , actual) = actual { equivalent = relaxed_equivalent_to_box (& Encoding :: Pointer (actual) , & expected) ; } } if ! equivalent { return Err (Inner :: MismatchedArgument (i , expected , actual . clone ()) . into ()) ; } } else { return Err (Inner :: MismatchedArgumentsCount (i , actual_count) . into ()) ; } } let remaining = iter . count () ; if remaining != 0 { return Err (Inner :: MismatchedArgumentsCount (actual_count + remaining , actual_count) . into ()) ; } let expected_count = method . name () . number_of_arguments () ; if expected_count != actual_count { return Err (Inner :: MismatchedArgumentsCount (expected_count , actual_count) . into ()) ; } Ok (()) }
};
}
