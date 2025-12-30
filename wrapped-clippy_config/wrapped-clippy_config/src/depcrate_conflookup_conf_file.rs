// Generated macro for lookup_conf_file (function)
macro_rules! Depcrate_conflookup_conf_file {
() => {
// Module: crate::conf
// Provides: {"lookup_conf_file"}
// Dependencies: {}
# [doc = " Search for the configuration file."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns any unexpected filesystem error encountered when searching for the config file"] pub fn lookup_conf_file () -> io :: Result < (Option < PathBuf > , Vec < String >) > { # [doc = " Possible filename to search for."] const CONFIG_FILE_NAMES : [& str ; 2] = [".clippy.toml" , "clippy.toml"] ; let mut current = env :: var_os ("CLIPPY_CONF_DIR") . or_else (| | env :: var_os ("CARGO_MANIFEST_DIR")) . map_or_else (| | PathBuf :: from (".") , PathBuf :: from) . canonicalize () ? ; let mut found_config : Option < PathBuf > = None ; let mut warnings = vec ! [] ; loop { for config_file_name in & CONFIG_FILE_NAMES { if let Ok (config_file) = current . join (config_file_name) . canonicalize () { match fs :: metadata (& config_file) { Err (e) if e . kind () == io :: ErrorKind :: NotFound => { } , Err (e) => return Err (e) , Ok (md) if md . is_dir () => { } , Ok (_) => { if let Some (ref found_config) = found_config { warnings . push (format ! ("using config file `{}`, `{}` will be ignored" , found_config . display () , config_file . display ())) ; } else { found_config = Some (config_file) ; } } , } } } if found_config . is_some () { return Ok ((found_config , warnings)) ; } if ! current . pop () { return Ok ((None , warnings)) ; } } }
};
}
