// Generated macro for impl_43 (impl)
macro_rules! Depcrate_expectimpl_43 {
() => {
// Module: crate::expect
// Provides: {"impl_43"}
// Dependencies: {}
# [cfg (feature = "async")] impl < T > AsyncExpect for & mut T where T : AsyncExpect , { async fn expect < N > (& mut self , needle : N) -> Result < Captures , Error > where N : Needle , { T :: expect (self , needle) . await } async fn check < N > (& mut self , needle : N) -> Result < Captures , Error > where N : Needle , { T :: check (self , needle) . await } async fn is_matched < N > (& mut self , needle : N) -> Result < bool , Error > where N : Needle , { T :: is_matched (self , needle) . await } async fn send < B > (& mut self , buf : B) -> Result < () , Error > where B : AsRef < [u8] > , { T :: send (self , buf) . await } async fn send_line < B > (& mut self , buf : B) -> Result < () , Error > where B : AsRef < [u8] > , { T :: send_line (self , buf) . await } }
};
}
