// Generated macro for impl_51 (impl)
macro_rules! Depcrate_configimpl_51 {
() => {
// Module: crate::config
// Provides: {"impl_51"}
// Dependencies: {}
impl ConfigFile { pub fn new (config_file : & Path) -> Result < Self , String > { let content = fs :: read_to_string (config_file) . map_err (| _ | { format ! ("Failed to read `{}`. Take a look at `Readme.md` to see how to set up the project" , config_file . display () ,) }) ? ; let toml = Toml :: parse (& content) . map_err (| err | { format ! ("Error occurred around `{}`: {:?}" , & content [err . start ..= err . end] , err . kind) }) ? ; let mut config = Self :: default () ; for (key , value) in toml . iter () { match (key , value) { ("gcc-path" , TomlValue :: String (value)) => { config . gcc_path = Some (value . as_str () . to_string ()) } ("gcc-path" , _) => { return failed_config_parsing (config_file , "Expected a string for `gcc-path`") ; } ("download-gccjit" , TomlValue :: Boolean (value)) => { config . download_gccjit = Some (* value) } ("download-gccjit" , _) => { return failed_config_parsing (config_file , "Expected a boolean for `download-gccjit`" ,) ; } _ => return failed_config_parsing (config_file , & format ! ("Unknown key `{key}`")) , } } match (config . gcc_path . as_mut () , config . download_gccjit) { (None , None | Some (false)) => { return failed_config_parsing (config_file , "At least one of `gcc-path` or `download-gccjit` value must be set" ,) ; } (Some (_) , Some (true)) => { println ! ("WARNING: both `gcc-path` and `download-gccjit` arguments are used, \
                    ignoring `gcc-path`") ; } (Some (gcc_path) , _) => { let path = Path :: new (gcc_path) ; * gcc_path = path . canonicalize () . map_err (| err | format ! ("Failed to get absolute path of `{gcc_path}`: {err:?}")) ? . display () . to_string () ; } _ => { } } Ok (config) } }
};
}
