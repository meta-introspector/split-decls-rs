// Generated macro for impl_591 (impl)
macro_rules! Depcrate_concurrency_genmc_configimpl_591 {
() => {
// Module: crate::concurrency::genmc::config
// Provides: {"impl_591"}
// Dependencies: {}
impl GenmcConfig { # [doc = " Function for parsing command line options for GenMC mode."] # [doc = ""] # [doc = " All GenMC arguments start with the string \"-Zmiri-genmc\"."] # [doc = " Passing any GenMC argument will enable GenMC mode."] # [doc = ""] # [doc = " `trimmed_arg` should be the argument to be parsed, with the suffix \"-Zmiri-genmc\" removed."] pub fn parse_arg (genmc_config : & mut Option < GenmcConfig > , trimmed_arg : & str ,) -> Result < () , String > { if genmc_config . is_none () { * genmc_config = Some (Default :: default ()) ; } if trimmed_arg . is_empty () { return Ok (()) ; } todo ! () ; } }
};
}
