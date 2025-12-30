// Generated macro for impl_45 (impl)
macro_rules! Depcrate_cliimpl_45 {
() => {
// Module: crate::cli
// Provides: {"impl_45"}
// Dependencies: {}
impl Verbosity { pub fn is_verbose (self) -> bool { matches ! (self , Verbosity :: Verbose | Verbosity :: Spammy) } pub fn is_spammy (self) -> bool { matches ! (self , Verbosity :: Spammy) } }
};
}
