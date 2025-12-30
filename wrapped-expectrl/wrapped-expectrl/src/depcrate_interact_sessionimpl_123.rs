// Generated macro for impl_123 (impl)
macro_rules! Depcrate_interact_sessionimpl_123 {
() => {
// Module: crate::interact::session
// Provides: {"impl_123"}
// Dependencies: {}
# [cfg (all (windows , feature = "async"))] impl < S , I , O , C > InteractSession < S , I , O , C > where I : AsyncRead + Unpin , O : AsyncWrite + Unpin , S : AsyncExpect + Healthcheck + AsyncWrite + AsyncRead + Unpin , { # [doc = " Runs the session."] # [doc = ""] # [doc = " See [`Session::interact`]."] # [doc = ""] # [doc = " [`Session::interact`]: crate::session::Session::interact"] pub async fn spawn (& mut self) -> Result < bool , Error > { interact_async (self) . await } }
};
}
