// Generated macro for Visibility (enum)
macro_rules! Depcrate_visibilityVisibility {
() => {
// Module: crate::visibility
// Provides: {"Visibility"}
// Dependencies: {}
# [doc = " Visibility of an item, with the path resolved."] # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash)] pub enum Visibility { # [doc = " Visibility is restricted to a certain module."] Module (ModuleId , VisibilityExplicitness) , # [doc = " Visibility is restricted to the crate."] PubCrate (Crate) , # [doc = " Visibility is unrestricted."] Public , }
};
}
