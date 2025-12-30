// Generated macro for get_rpaths (function)
macro_rules! Depcrate_back_rpathget_rpaths {
() => {
// Module: crate::back::rpath
// Provides: {"get_rpaths"}
// Dependencies: {}
fn get_rpaths (config : & RPathConfig < '_ >) -> Vec < OsString > { debug ! ("output: {:?}" , config . out_filename . display ()) ; debug ! ("libs:") ; for libpath in config . libs { debug ! ("    {:?}" , libpath . display ()) ; } let rpaths = get_rpaths_relative_to_output (config) ; debug ! ("rpaths:") ; for rpath in & rpaths { debug ! ("    {:?}" , rpath) ; } minimize_rpaths (& rpaths) }
};
}
