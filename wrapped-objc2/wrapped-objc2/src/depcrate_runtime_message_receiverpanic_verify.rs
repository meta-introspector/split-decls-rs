// Generated macro for panic_verify (function)
macro_rules! Depcrate_runtime_message_receiverpanic_verify {
() => {
// Module: crate::runtime::message_receiver
// Provides: {"panic_verify"}
// Dependencies: {}
# [cfg (debug_assertions)] # [track_caller] fn panic_verify (cls : & AnyClass , sel : Sel , err : & crate :: runtime :: VerificationError) -> ! { panic ! ("invalid message send to {}[{cls} {sel}]: {err}" , if cls . is_metaclass () { "+" } else { "-" } ,) }
};
}
