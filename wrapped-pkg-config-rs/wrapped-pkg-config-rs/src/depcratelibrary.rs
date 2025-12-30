// Generated macro for Library (struct)
macro_rules! DepcrateLibrary {
() => {
// Module: crate
// Provides: {"Library"}
// Dependencies: {}
# [derive (Clone , Debug)] pub struct Library { # [doc = " Libraries specified by -l"] pub libs : Vec < String > , # [doc = " Library search paths specified by -L"] pub link_paths : Vec < PathBuf > , # [doc = " Library file paths specified without -l"] pub link_files : Vec < PathBuf > , # [doc = " Darwin frameworks specified by -framework"] pub frameworks : Vec < String > , # [doc = " Darwin framework search paths specified by -F"] pub framework_paths : Vec < PathBuf > , # [doc = " C/C++ header include paths specified by -I"] pub include_paths : Vec < PathBuf > , # [doc = " Linker options specified by -Wl"] pub ld_args : Vec < Vec < String > > , # [doc = " C/C++ definitions specified by -D"] pub defines : HashMap < String , Option < String > > , # [doc = " Version specified by .pc file's Version field"] pub version : String , # [doc = " Ensure that this struct can only be created via its private `[Library::new]` constructor."] # [doc = " Users of this crate can only access the struct via `[Config::probe]`."] _priv : () , }
};
}
