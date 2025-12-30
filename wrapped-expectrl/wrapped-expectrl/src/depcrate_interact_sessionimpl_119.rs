// Generated macro for impl_119 (impl)
macro_rules! Depcrate_interact_sessionimpl_119 {
() => {
// Module: crate::interact::session
// Provides: {"impl_119"}
// Dependencies: {}
# [cfg (all (unix , not (any (feature = "async" , feature = "polling"))))] impl < S , I , O , C > InteractSession < S , I , O , C > where I : Read , O : Write , S : Expect + Termios + Healthcheck < Status = WaitStatus > + NonBlocking + Write + Read , { # [doc = " Runs the session."] # [doc = ""] # [doc = " See [`Session::interact`]."] # [doc = ""] # [doc = " [`Session::interact`]: crate::session::Session::interact"] pub fn spawn (& mut self) -> ExpectResult < bool > { let is_echo = self . session . is_echo () ? ; if ! is_echo { let _ = self . session . set_echo (true) ; } self . status = None ; let is_alive = interact_buzy_loop (self) ? ; if ! is_echo { let _ = self . session . set_echo (false) ; } Ok (is_alive) } }
};
}
