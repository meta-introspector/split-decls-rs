// Generated macro for impl_122 (impl)
macro_rules! Depcrate_interact_sessionimpl_122 {
() => {
// Module: crate::interact::session
// Provides: {"impl_122"}
// Dependencies: {}
# [cfg (all (unix , feature = "async"))] impl < S , I , O , C > InteractSession < S , I , O , C > where I : AsyncRead + Unpin , O : AsyncWrite + Unpin , S : AsyncExpect + Termios + Healthcheck < Status = WaitStatus > + AsyncWrite + AsyncRead + Unpin , { # [doc = " Runs the session."] # [doc = ""] # [doc = " See [`Session::interact`]."] # [doc = ""] # [doc = " [`Session::interact`]: crate::session::Session::interact"] pub async fn spawn (& mut self) -> Result < bool , Error > { let is_echo = self . session . is_echo () . map_err (Error :: IO) ? ; if ! is_echo { let _ = self . session . set_echo (true) ; } let is_alive = interact_async (self) . await ? ; if ! is_echo { let _ = self . session . set_echo (false) ; } Ok (is_alive) } }
};
}
