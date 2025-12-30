// Generated macro for generate (function)
macro_rules! Depcrate_fuzzgenerate {
() => {
// Module: crate::fuzz
// Provides: {"generate"}
// Dependencies: {}
# [doc = " Generates a new rustlantis file for us to run tests on."] fn generate (seed : u64 , print_tmp_vars : bool) -> Result < std :: path :: PathBuf , String > { use std :: io :: Write ; let mut out_path = std :: env :: temp_dir () ; out_path . push (format ! ("fuzz{seed}.rs")) ; let mut generate = std :: process :: Command :: new ("cargo") ; generate . args (["run" , "--release" , "--bin" , "generate"]) . arg (format ! ("{seed}")) . current_dir ("clones/rustlantis") ; if print_tmp_vars { generate . arg ("--debug") ; } let out = generate . output () . map_err (| err | format ! ("{err:?}")) ? ; std :: fs :: File :: create (& out_path) . map_err (| err | format ! ("{err:?}")) ? . write_all (& out . stdout) . map_err (| err | format ! ("{err:?}")) ? ; Ok (out_path) }
};
}
