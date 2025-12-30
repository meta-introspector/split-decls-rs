// Generated macro for impl_233 (impl)
macro_rules! Depcrate_replimpl_233 {
() => {
// Module: crate::repl
// Provides: {"impl_233"}
// Dependencies: {}
# [cfg (feature = "async")] impl < S > AsyncExpect for ReplSession < S > where S : AsyncExpect , { async fn expect < N > (& mut self , needle : N) -> Result < Captures , Error > where N : Needle , { S :: expect (self . get_session_mut () , needle) . await } async fn check < N > (& mut self , needle : N) -> Result < Captures , Error > where N : Needle , { S :: check (self . get_session_mut () , needle) . await } async fn is_matched < N > (& mut self , needle : N) -> Result < bool , Error > where N : Needle , { S :: is_matched (self . get_session_mut () , needle) . await } async fn send < B > (& mut self , buf : B) -> Result < () , Error > where B : AsRef < [u8] > , { S :: send (self . get_session_mut () , buf) . await } async fn send_line < B > (& mut self , buf : B) -> Result < () , Error > where B : AsRef < [u8] > , { S :: send_line (self . get_session_mut () , buf) . await } }
};
}
