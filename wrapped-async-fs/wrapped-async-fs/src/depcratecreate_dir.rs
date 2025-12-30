// Generated macro for create_dir (function)
macro_rules! Depcratecreate_dir {
() => {
// Module: crate
// Provides: {"create_dir"}
// Dependencies: {}
# [doc = " Creates a new, empty directory at the provided path"] # [doc = ""] # [doc = " # Platform-specific behavior"] # [doc = ""] # [doc = " This function currently corresponds to the `mkdir` function on Unix"] # [doc = " and the `CreateDirectory` function on Windows."] # [doc = " Note that this [may change in the future][changes]."] # [doc = ""] # [doc = " [changes]: io#platform-specific-behavior"] # [doc = ""] # [doc = " Note that if a parent of the given path doesn't exist, this function will"] # [doc = " return an error. To create a directory and all its missing parents at the"] # [doc = " same time, use the [`create_dir_all`] function."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return an error in the following situations, but is not"] # [doc = " limited to just these cases:"] # [doc = ""] # [doc = " * User lacks permissions to create directory at `path`."] # [doc = " * A parent of the given path doesn't exist. (To create a directory and all"] # [doc = "   its missing parents at the same time, use the [`create_dir_all`]"] # [doc = "   function.)"] # [doc = " * `path` already exists."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::fs;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     fs::create_dir(\"/some/dir\")?;"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] pub async fn create_dir < P : AsRef < Path > > (path : P) -> io :: Result < () > { let path = path . as_ref () . to_owned () ; unblock (move | | std :: fs :: create_dir (path)) . await }
};
}
