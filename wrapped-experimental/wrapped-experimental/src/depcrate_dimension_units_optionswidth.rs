// Generated macro for Width (enum)
macro_rules! Depcrate_dimension_units_optionsWidth {
() => {
// Module: crate::dimension::units::options
// Provides: {"Width"}
// Dependencies: {}
# [derive (Debug , Eq , PartialEq , Clone , Copy , Default)] # [non_exhaustive] pub enum Width { # [doc = " Format the units with the long units format."] # [doc = ""] # [doc = " For example, 1 hour formats as \"1 hour\" in en-US."] Long , # [doc = " Format the units with the short units format."] # [doc = ""] # [doc = " For example, 1 hour formats as \"1 hr\" in en-US."] # [default] Short , # [doc = " Format the units with the narrow units format."] # [doc = ""] # [doc = " The narrow symbol may be ambiguous, so it should be evident from context which"] # [doc = " units is being represented."] # [doc = ""] # [doc = " For example, 1 hour formats as \"1 h\" in most locales."] Narrow , }
};
}
