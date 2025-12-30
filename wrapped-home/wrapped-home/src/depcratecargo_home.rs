// Generated macro for cargo_home (function)
macro_rules! Depcratecargo_home {
() => {
// Module: crate
// Provides: {"cargo_home"}
// Dependencies: {}
# [doc = " Returns the storage directory used by Cargo, often known as"] # [doc = " `.cargo` or `CARGO_HOME`."] # [doc = ""] # [doc = " It returns one of the following values, in this order of"] # [doc = " preference:"] # [doc = ""] # [doc = " - The value of the `CARGO_HOME` environment variable, if it is"] # [doc = "   an absolute path."] # [doc = " - The value of the current working directory joined with the value"] # [doc = "   of the `CARGO_HOME` environment variable, if `CARGO_HOME` is a"] # [doc = "   relative directory."] # [doc = " - The `.cargo` directory in the user's home directory, as reported"] # [doc = "   by the `home_dir` function."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function fails if it fails to retrieve the current directory,"] # [doc = " or if the home directory cannot be determined."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " match home::cargo_home() {"] # [doc = "     Ok(path) => println!(\"{}\", path.display()),"] # [doc = "     Err(err) => eprintln!(\"Cannot get your cargo home dir: {:?}\", err),"] # [doc = " }"] # [doc = " ```"] pub fn cargo_home () -> io :: Result < PathBuf > { env :: cargo_home_with_env (& env :: OS_ENV) }
};
}
