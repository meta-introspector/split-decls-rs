// Generated macro for FoundCrate (enum)
macro_rules! DepcrateFoundCrate {
() => {
// Module: crate
// Provides: {"FoundCrate"}
// Dependencies: {}
# [doc = " The crate as found by [`crate_name`]."] # [derive (Debug , PartialEq , Clone , Eq)] pub enum FoundCrate { # [doc = " The searched crate is this crate itself."] Itself , # [doc = " The searched crate was found with this name."] Name (String) , }
};
}
