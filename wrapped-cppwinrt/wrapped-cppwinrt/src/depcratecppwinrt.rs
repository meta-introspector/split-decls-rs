// Generated macro for cppwinrt (function)
macro_rules! Depcratecppwinrt {
() => {
// Module: crate
// Provides: {"cppwinrt"}
// Dependencies: {}
# [doc = " Calls the C++/WinRT compiler with the given arguments."] # [doc = ""] # [doc = " Use `cppwinrt([\"-help\"])` for available options."] # [track_caller] pub fn cppwinrt < I , S > (args : I) -> String where I : IntoIterator < Item = S > , S : AsRef < std :: ffi :: OsStr > , { let mut path = std :: env :: temp_dir () ; path . push (unique ()) ; std :: fs :: create_dir_all (& path) . unwrap () ; path . push ("cppwinrt.exe") ; std :: fs :: write (& path , std :: include_bytes ! ("../cppwinrt.exe")) . unwrap () ; let mut command = std :: process :: Command :: new (& path) ; command . args (args) ; let output = command . output () . expect ("failed to run cppwinrt") ; _ = std :: fs :: remove_file (path) ; if output . status . success () { String :: from_utf8_lossy (& output . stdout) . to_string () } else { panic ! ("{}" , String :: from_utf8_lossy (& output . stderr)) } }
};
}
