// Generated macro for msg_send_check (function)
macro_rules! Depcrate_runtime_message_receivermsg_send_check {
() => {
// Module: crate::runtime::message_receiver
// Provides: {"msg_send_check"}
// Dependencies: {}
# [doc = " Help with monomorphizing in framework crates"] # [cfg (debug_assertions)] # [track_caller] fn msg_send_check (obj : Option < & AnyObject > , sel : Sel , args : & [crate :: encode :: Encoding] , ret : & crate :: encode :: Encoding ,) { let cls = if let Some (obj) = obj { obj . class () } else { panic_null (sel) } ; msg_send_check_class (cls , sel , args , ret) ; }
};
}
