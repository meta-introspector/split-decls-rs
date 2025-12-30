// Generated macro for impl_120 (impl)
macro_rules! Depcrate_interact_sessionimpl_120 {
() => {
// Module: crate::interact::session
// Provides: {"impl_120"}
// Dependencies: {}
# [cfg (all (windows , not (any (feature = "async" , feature = "polling"))))] impl < S , I , O , C > InteractSession < S , I , O , C > where I : Read , O : Write , S : Expect + Healthcheck + NonBlocking + Write + Read , { # [doc = " Runs the session."] # [doc = ""] # [doc = " See [`Session::interact`]."] # [doc = ""] # [doc = " [`Session::interact`]: crate::session::Session::interact"] pub fn spawn (& mut self) -> ExpectResult < bool > { interact_buzy_loop (self) } }
};
}
