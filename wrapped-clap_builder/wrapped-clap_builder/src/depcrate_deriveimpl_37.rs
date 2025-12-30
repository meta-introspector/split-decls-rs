// Generated macro for impl_37 (impl)
macro_rules! Depcrate_deriveimpl_37 {
() => {
// Module: crate::derive
// Provides: {"impl_37"}
// Dependencies: {}
impl < T : FromArgMatches > FromArgMatches for Box < T > { fn from_arg_matches (matches : & ArgMatches) -> Result < Self , Error > { < T as FromArgMatches > :: from_arg_matches (matches) . map (Box :: new) } fn from_arg_matches_mut (matches : & mut ArgMatches) -> Result < Self , Error > { < T as FromArgMatches > :: from_arg_matches_mut (matches) . map (Box :: new) } fn update_from_arg_matches (& mut self , matches : & ArgMatches) -> Result < () , Error > { < T as FromArgMatches > :: update_from_arg_matches (self , matches) } fn update_from_arg_matches_mut (& mut self , matches : & mut ArgMatches) -> Result < () , Error > { < T as FromArgMatches > :: update_from_arg_matches_mut (self , matches) } }
};
}
