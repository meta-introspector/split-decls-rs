// Generated macro for write_msg_text (function)
macro_rules! Depcrate_msgwrite_msg_text {
() => {
// Module: crate::msg
// Provides: {"write_msg_text"}
// Dependencies: {}
fn write_msg_text (out : & mut dyn Write , msg : & str) -> io :: Result < () > { log :: debug ! ("> {msg}") ; write ! (out , "Content-Length: {}\r\n\r\n" , msg . len ()) ? ; out . write_all (msg . as_bytes ()) ? ; out . flush () ? ; Ok (()) }
};
}
