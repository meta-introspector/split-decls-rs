// Generated macro for impl_232 (impl)
macro_rules! Depcrate_replimpl_232 {
() => {
// Module: crate::repl
// Provides: {"impl_232"}
// Dependencies: {}
impl < S > Expect for ReplSession < S > where S : Expect , { fn expect < N > (& mut self , needle : N) -> Result < Captures , Error > where N : Needle , { S :: expect (self . get_session_mut () , needle) } fn check < N > (& mut self , needle : N) -> Result < Captures , Error > where N : Needle , { S :: check (self . get_session_mut () , needle) } fn is_matched < N > (& mut self , needle : N) -> Result < bool , Error > where N : Needle , { S :: is_matched (self . get_session_mut () , needle) } fn send < B > (& mut self , buf : B) -> Result < () , Error > where B : AsRef < [u8] > , { S :: send (self . get_session_mut () , buf) } fn send_line < B > (& mut self , buf : B) -> Result < () , Error > where B : AsRef < [u8] > , { S :: send_line (self . get_session_mut () , buf) } }
};
}
