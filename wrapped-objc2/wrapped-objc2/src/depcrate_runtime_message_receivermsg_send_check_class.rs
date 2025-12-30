// Generated macro for msg_send_check_class (function)
macro_rules! Depcrate_runtime_message_receivermsg_send_check_class {
() => {
// Module: crate::runtime::message_receiver
// Provides: {"msg_send_check_class"}
// Dependencies: {}
# [cfg (debug_assertions)] # [track_caller] fn msg_send_check_class (cls : & AnyClass , sel : Sel , args : & [crate :: encode :: Encoding] , ret : & crate :: encode :: Encoding ,) { if cfg ! (feature = "disable-encoding-assertions") { return ; } use super :: verify :: { verify_method_signature , Inner , VerificationError } ; let err = if let Some (method) = cls . instance_method (sel) { if let Err (err) = verify_method_signature (method , args , ret) { err } else { return ; } } else { VerificationError :: from (Inner :: MethodNotFound) } ; panic_verify (cls , sel , & err) ; }
};
}
