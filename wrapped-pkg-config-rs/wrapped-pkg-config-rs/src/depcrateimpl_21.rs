// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { match * self { Error :: EnvNoPkgConfig (ref name) => write ! (f , "Aborted because {} is set" , name) , Error :: CrossCompilation => f . write_str ("pkg-config has not been configured to support cross-compilation.\n\
                \n\
                Install a sysroot for the target platform and configure it via\n\
                PKG_CONFIG_SYSROOT_DIR and PKG_CONFIG_PATH, or install a\n\
                cross-compiling wrapper for pkg-config and set it via\n\
                PKG_CONFIG environment variable." ,) , Error :: Command { ref command , ref cause , } => { match cause . kind () { io :: ErrorKind :: NotFound => { let crate_name = std :: env :: var ("CARGO_PKG_NAME") . unwrap_or_else (| _ | "sys" . to_owned ()) ; let instructions = if cfg ! (target_os = "macos") { "Try `brew install pkgconf` if you have Homebrew.\n" } else if cfg ! (target_os = "ios") { "" } else if cfg ! (unix) { "Try `apt install pkg-config`, or `yum install pkg-config`, or `brew install pkgconf`\n\
                            or `pkg install pkg-config`, or `apk add pkgconfig` \
                            depending on your distribution.\n" } else { "" } ; write ! (f , "Could not run `{command}`\n\
                        The pkg-config command could not be found.\n\
                        \n\
                        Most likely, you need to install a pkg-config package for your OS.\n\
                        {instructions}\
                        \n\
                        If you've already installed it, ensure the pkg-config command is one of the\n\
                        directories in the PATH environment variable.\n\
                        \n\
                        If you did not expect this build to link to a pre-installed system library,\n\
                        then check documentation of the {crate_name} crate for an option to\n\
                        build the library from source, or disable features or dependencies\n\
                        that require pkg-config." , command = command , instructions = instructions , crate_name = crate_name) } _ => write ! (f , "Failed to run command `{}`, because: {}" , command , cause) , } } Error :: ProbeFailure { ref name , ref command , ref output , } => { let crate_name = env :: var ("CARGO_PKG_NAME") . unwrap_or (String :: from ("<NO CRATE NAME>")) ; writeln ! (f , "") ? ; writeln ! (f , "pkg-config {}" , match output . status . code () { Some (code) => format ! ("exited with status code {}" , code) , None => "was terminated by signal" . to_string () , }) ? ; writeln ! (f , "> {}\n" , command) ? ; writeln ! (f , "The system library `{}` required by crate `{}` was not found." , name , crate_name) ? ; writeln ! (f , "The file `{}.pc` needs to be installed and the PKG_CONFIG_PATH environment variable must contain its parent directory." , name) ? ; if let Some (_code) = output . status . code () { let search_locations = ["PKG_CONFIG_PATH_FOR_TARGET" , "PKG_CONFIG_PATH"] ; let mut search_data = None ; for location in search_locations . iter () { if let Ok (search_path) = env :: var (location) { search_data = Some ((location , search_path)) ; break ; } } let hint = if let Some ((search_location , search_path)) = search_data { writeln ! (f , "{} contains the following:\n{}" , search_location , search_path . split (':') . map (| path | format ! ("    - {}" , path)) . collect ::< Vec < String >> () . join ("\n") ,) ? ; format ! ("you may need to install a package such as {name}, {name}-dev or {name}-devel." , name = name) } else { writeln ! (f , "The PKG_CONFIG_PATH environment variable is not set.") ? ; format ! ("if you have installed the library, try setting PKG_CONFIG_PATH to the directory containing `{}.pc`." , name) } ; writeln ! (f , "\nHINT: {}" , hint) ? ; } Ok (()) } Error :: Failure { ref command , ref output , } => { write ! (f , "`{}` did not exit successfully: {}" , command , output . status) ? ; format_output (output , f) } Error :: __Nonexhaustive => panic ! () , } } }
};
}
