// Generated macro for _message_prettify (function)
macro_rules! Depcrate_message_message_prettify {
() => {
// Module: crate::message
// Provides: {"_message_prettify"}
// Dependencies: {}
fn _message_prettify (message : CString , comment_char : Option < u8 >) -> Result < String , Error > { let ret = Buf :: new () ; unsafe { try_call ! (raw :: git_message_prettify (ret . raw () , message , comment_char . is_some () as c_int , comment_char . unwrap_or (0) as c_char)) ; } Ok (ret . as_str () . unwrap () . to_string ()) }
};
}
