// Generated macro for conditional_try (macro)
macro_rules! Depcrate_runtime_message_receiverconditional_try {
() => {
// Module: crate::runtime::message_receiver
// Provides: {"conditional_try"}
// Dependencies: {}
# [cfg (feature = "catch-all")] macro_rules ! conditional_try { (|| $ expr : expr) => { { let f = core :: panic :: AssertUnwindSafe (|| $ expr) ; match crate :: exception :: catch (f) { Ok (r) => r , Err (exception) => { if let Some (exception) = exception { panic ! ("uncaught {exception:?}\n{}" , exception . stack_trace ()) } else { panic ! ("uncaught exception nil") } } } } } ; }
};
}
