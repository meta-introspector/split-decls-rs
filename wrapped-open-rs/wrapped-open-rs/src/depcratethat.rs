// Generated macro for that (function)
macro_rules! Depcratethat {
() => {
// Module: crate
// Provides: {"that"}
// Dependencies: {}
# [doc = " Open path with the default application without blocking."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " let path = \"http://rust-lang.org\";"] # [doc = ""] # [doc = " match open::that(path) {"] # [doc = "     Ok(()) => println!(\"Opened '{}' successfully.\", path),"] # [doc = "     Err(err) => panic!(\"An error occurred when opening '{}': {}\", path, err),"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " A [`std::io::Error`] is returned on failure. Because different operating systems"] # [doc = " handle errors differently it is recommend to not match on a certain error."] # [doc = ""] # [doc = " # Beware"] # [doc = ""] # [doc = " Sometimes, depending on the platform and system configuration, launchers *can* block."] # [doc = " If you want to be sure they don't, use [`that_in_background()`] or [`that_detached`] instead."] pub fn that (path : impl AsRef < OsStr >) -> io :: Result < () > { let mut last_err = None ; for mut cmd in commands (path) { match cmd . status_without_output () { Ok (status) => { return Ok (status) . into_result (& cmd) ; } Err (err) => last_err = Some (err) , } } Err (last_err . expect ("no launcher worked, at least one error")) }
};
}
