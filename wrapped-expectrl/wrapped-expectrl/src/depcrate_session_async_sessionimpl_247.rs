// Generated macro for impl_247 (impl)
macro_rules! Depcrate_session_async_sessionimpl_247 {
() => {
// Module: crate::session::async_session
// Provides: {"impl_247"}
// Dependencies: {}
impl < P , S > AsyncExpect for Session < P , S > where S : AsyncWrite + AsyncRead + Unpin , { async fn expect < N > (& mut self , needle : N) -> Result < Captures , Error > where N : Needle , { match self . stream . expect_lazy { true => self . stream . expect_lazy (needle) . await , false => self . stream . expect_gready (needle) . await , } } async fn check < N > (& mut self , needle : N) -> Result < Captures , Error > where N : Needle , { self . stream . check (needle) . await } async fn is_matched < N > (& mut self , needle : N) -> Result < bool , Error > where N : Needle , { self . stream . is_matched (needle) . await } async fn send < B > (& mut self , buf : B) -> Result < () , Error > where B : AsRef < [u8] > , { self . stream . write_all (buf . as_ref ()) . await . map_err (Error :: IO) } async fn send_line < B > (& mut self , buf : B) -> Result < () , Error > where B : AsRef < [u8] > , { # [cfg (windows)] const LINE_ENDING : & [u8] = b"\r\n" ; # [cfg (not (windows))] const LINE_ENDING : & [u8] = b"\n" ; self . stream . write_all (buf . as_ref ()) . await ? ; self . stream . write_all (LINE_ENDING) . await ? ; Ok (()) } }
};
}
