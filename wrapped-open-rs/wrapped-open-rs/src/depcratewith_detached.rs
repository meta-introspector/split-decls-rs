// Generated macro for with_detached (function)
macro_rules! Depcratewith_detached {
() => {
// Module: crate
// Provides: {"with_detached"}
// Dependencies: {}
# [doc = " Open path with the given application using a detached process, which is useful if"] # [doc = " the program ends up to be blocking or want to out-live your app. Otherwise, prefer [`with()`] for"] # [doc = " straightforward error handling."] # [doc = ""] # [doc = " See documentation of [`with()`] for more details."] pub fn with_detached < T : AsRef < OsStr > > (path : T , app : impl Into < String >) -> io :: Result < () > { # [cfg (any (not (feature = "shellexecute-on-windows") , not (windows)))] { let mut cmd = with_command (path , app) ; cmd . spawn_detached () } # [cfg (all (windows , feature = "shellexecute-on-windows"))] { windows :: with_detached (path , app) } }
};
}
