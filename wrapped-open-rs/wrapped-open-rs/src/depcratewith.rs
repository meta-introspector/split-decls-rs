// Generated macro for with (function)
macro_rules! Depcratewith {
() => {
// Module: crate
// Provides: {"with"}
// Dependencies: {}
# [doc = " Open path with the given application."] # [doc = ""] # [doc = " This function may block if the application or launcher doesn't detach itself."] # [doc = " In that case, consider using [`with_in_background()`] or [`with_command()]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " let path = \"http://rust-lang.org\";"] # [doc = " let app = \"firefox\";"] # [doc = ""] # [doc = " match open::with(path, app) {"] # [doc = "     Ok(()) => println!(\"Opened '{}' successfully.\", path),"] # [doc = "     Err(err) => panic!(\"An error occurred when opening '{}': {}\", path, err),"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " A [`std::io::Error`] is returned on failure. Because different operating systems"] # [doc = " handle errors differently it is recommend to not match on a certain error."] pub fn with (path : impl AsRef < OsStr > , app : impl Into < String >) -> io :: Result < () > { let mut cmd = with_command (path , app) ; cmd . status_without_output () . into_result (& cmd) }
};
}
