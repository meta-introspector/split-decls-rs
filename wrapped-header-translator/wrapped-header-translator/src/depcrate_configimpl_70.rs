// Generated macro for impl_70 (impl)
macro_rules! Depcrate_configimpl_70 {
() => {
// Module: crate::config
// Provides: {"impl_70"}
// Dependencies: {}
impl LibraryConfig { pub fn from_file (file : & Path) -> Result < Self , Box < dyn Error > > { let s = fs :: read_to_string (file) ? ; let config : Self = toml :: from_str (& s) ? ; assert_eq ! (config . framework . to_lowercase () , config . krate . replace ("objc2-" , "") . replace ('-' , "") , "crate name had an unexpected format" ,) ; if matches ! (&* config . krate , "objc2-tv-ml-kit" | "objc2-tv-ui-kit") { return Ok (config) ; } if config . krate == "objc2-xc-ui-automation" { return Ok (config) ; } if config . krate == "objc2-open-gl-es" { return Ok (config) ; } if config . krate == "objc2-javascript-core" { return Ok (config) ; } if config . krate == "objc2-itunes-library" { return Ok (config) ; } if config . krate == "objc2-io-usb-host" { return Ok (config) ; } assert_eq ! (Some (&* config . framework . to_train_case () . to_lowercase ()) , config . krate . strip_prefix ("objc2-") , "crate name had an unexpected format" ,) ; Ok (config) } }
};
}
