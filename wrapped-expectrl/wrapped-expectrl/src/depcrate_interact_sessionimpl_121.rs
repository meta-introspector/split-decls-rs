// Generated macro for impl_121 (impl)
macro_rules! Depcrate_interact_sessionimpl_121 {
() => {
// Module: crate::interact::session
// Provides: {"impl_121"}
// Dependencies: {}
# [cfg (all (unix , not (feature = "async") , feature = "polling"))] impl < S , I , O , C > InteractSession < S , I , O , C > where I : Read + std :: os :: unix :: io :: AsRawFd , O : Write , S : Expect + Termios + Healthcheck < Status = WaitStatus > + Write + Read + std :: os :: unix :: io :: AsRawFd , { # [doc = " Runs the session."] # [doc = ""] # [doc = " See [`Session::interact`]."] # [doc = ""] # [doc = " [`Session::interact`]: crate::session::Session::interact"] pub fn spawn (& mut self) -> ExpectResult < bool > { # [cfg (unix)] { let is_echo = self . session . is_echo () ? ; if ! is_echo { let _ = self . session . set_echo (true) ; } self . status = None ; let is_alive = interact_polling (self) ? ; if ! is_echo { let _ = self . session . set_echo (false) ; } Ok (is_alive) } # [cfg (windows)] { interact_buzy_loop (self) } } }
};
}
