// Generated macro for impl_250 (impl)
macro_rules! Depcrate_data_output_countimpl_250 {
() => {
// Module: crate::data::output::count
// Provides: {"impl_250"}
// Dependencies: {}
impl PackLocation { # [doc = " Directly go through to `LookedUp` variant, panic otherwise"] pub fn is_none (& self) -> bool { match self { PackLocation :: LookedUp (opt) => opt . is_none () , PackLocation :: NotLookedUp => unreachable ! ("must have been resolved") , } } # [doc = " Directly go through to `LookedUp` variant, panic otherwise"] pub fn as_ref (& self) -> Option < & crate :: data :: entry :: Location > { match self { PackLocation :: LookedUp (opt) => opt . as_ref () , PackLocation :: NotLookedUp => unreachable ! ("must have been resolved") , } } }
};
}
