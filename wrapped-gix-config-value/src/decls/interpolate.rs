macro_rules! deps {
    () => {
        Path!();
        Error!();
    };
}

macro_rules! interpolate {
    () => {
        deps!();
        # [doc = ""] pub mod interpolate { use std :: path :: PathBuf ; # [doc = " Options for interpolating paths with [`Path::interpolate()`][crate::Path::interpolate()]."] # [derive (Clone , Copy)] pub struct Context < 'a > { # [doc = " The location where gitoxide or git is installed. If `None`, `%(prefix)` in paths will cause an error."] pub git_install_dir : Option < & 'a std :: path :: Path > , # [doc = " The home directory of the current user. If `None`, `~/` in paths will cause an error."] pub home_dir : Option < & 'a std :: path :: Path > , # [doc = " A function returning the home directory of a given user. If `None`, `~name/` in paths will cause an error."] pub home_for_user : Option < fn (& str) -> Option < PathBuf > > , } impl Default for Context < '_ > { fn default () -> Self { Context { git_install_dir : None , home_dir : None , home_for_user : Some (home_for_user) , } } } # [doc = " The error returned by [`Path::interpolate()`][crate::Path::interpolate()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("{} is missing" , . what)] Missing { what : & 'static str } , # [error ("Ill-formed UTF-8 in {}" , . what)] Utf8Conversion { what : & 'static str , # [source] err : gix_path :: Utf8Error , } , # [error ("Ill-formed UTF-8 in username")] UsernameConversion (# [from] std :: str :: Utf8Error) , # [error ("User interpolation is not available on this platform")] UserInterpolationUnsupported , } # [doc = " Obtain the home directory for the given user `name` or return `None` if the user wasn't found"] # [doc = " or any other error occurred."] # [doc = " It can be used as `home_for_user` parameter in [`Path::interpolate()`][crate::Path::interpolate()]."] # [cfg_attr (windows , allow (unused_variables))] # [cfg_attr (all (target_family = "wasm" , not (target_os = "emscripten")) , allow (unused_variables))] pub fn home_for_user (name : & str) -> Option < PathBuf > { # [cfg (not (any (target_os = "android" , target_os = "windows" , all (target_family = "wasm" , not (target_os = "emscripten")))))] { let cname = std :: ffi :: CString :: new (name) . ok () ? ; # [allow (unsafe_code)] let pwd = unsafe { libc :: getpwnam (cname . as_ptr ()) } ; if pwd . is_null () { None } else { use std :: os :: unix :: ffi :: OsStrExt ; # [allow (unsafe_code)] let cstr = unsafe { std :: ffi :: CStr :: from_ptr ((* pwd) . pw_dir) } ; Some (std :: ffi :: OsStr :: from_bytes (cstr . to_bytes ()) . into ()) } } # [cfg (any (target_os = "android" , target_os = "windows" , all (target_family = "wasm" , not (target_os = "emscripten"))))] { None } } }
    };
}

interpolate!();