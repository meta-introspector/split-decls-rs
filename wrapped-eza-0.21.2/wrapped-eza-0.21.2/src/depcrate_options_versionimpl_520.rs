// Generated macro for impl_520 (impl)
macro_rules! Depcrate_options_versionimpl_520 {
() => {
// Module: crate::options::version
// Provides: {"impl_520"}
// Dependencies: {}
impl fmt :: Display for VersionString { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { write ! (f , "{}" , include_str ! (concat ! (env ! ("OUT_DIR") , "/version_string.txt"))) } }
};
}
