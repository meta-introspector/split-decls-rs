// Generated macro for impl_124 (impl)
macro_rules! Depcrate_interact_sessionimpl_124 {
() => {
// Module: crate::interact::session
// Provides: {"impl_124"}
// Dependencies: {}
# [cfg (all (windows , feature = "polling" , not (feature = "async")))] impl < I , O , C > InteractSession < crate :: session :: OsSession , I , O , C > where I : Read + Clone + Send + 'static , O : Write , { # [doc = " Runs the session."] # [doc = ""] # [doc = " See [`Session::interact`]."] # [doc = ""] # [doc = " [`Session::interact`]: crate::session::Session::interact"] pub fn spawn (& mut self) -> Result < bool , Error > { interact_polling_on_thread (self) } }
};
}
