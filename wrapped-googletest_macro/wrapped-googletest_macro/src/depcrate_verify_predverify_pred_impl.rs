// Generated macro for verify_pred_impl (function)
macro_rules! Depcrate_verify_predverify_pred_impl {
() => {
// Module: crate::verify_pred
// Provides: {"verify_pred_impl"}
// Dependencies: {}
pub fn verify_pred_impl (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { let parsed = parse_macro_input ! (input as Expr) ; let error_message = quote ! (# parsed) . to_string () + " was false with" ; let mut state = AccumulatePartsState :: new () ; let pred_value = state . accumulate_parts (parsed) ; let AccumulatePartsState { error_message_ident , mut statements , .. } = state ; let _ = statements . pop () ; quote ! { { let mut # error_message_ident = # error_message . to_string () ; # (# statements) * if (# pred_value) { Ok (()) } else { :: core :: result :: Result :: Err (:: googletest :: internal :: test_outcome :: TestAssertionFailure :: create (# error_message_ident)) } } } . into () }
};
}
