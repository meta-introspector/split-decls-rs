// Generated macro for that_detached (function)
macro_rules! Depcratethat_detached {
() => {
// Module: crate
// Provides: {"that_detached"}
// Dependencies: {}
# [doc = " Open path with the default application using a detached process. which is useful if"] # [doc = " the program ends up to be blocking or want to out-live your app"] # [doc = ""] # [doc = " See documentation of [`that()`] for more details."] pub fn that_detached (path : impl AsRef < OsStr >) -> io :: Result < () > { # [cfg (any (not (feature = "shellexecute-on-windows") , not (windows)))] { let mut last_err = None ; for mut cmd in commands (path) { match cmd . spawn_detached () { Ok (_) => { return Ok (()) ; } Err (err) => last_err = Some (err) , } } Err (last_err . expect ("no launcher worked, at least one error")) } # [cfg (all (windows , feature = "shellexecute-on-windows"))] { windows :: that_detached (path) } }
};
}
