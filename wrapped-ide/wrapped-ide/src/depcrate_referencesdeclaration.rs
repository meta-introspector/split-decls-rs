// Generated macro for Declaration (struct)
macro_rules! Depcrate_referencesDeclaration {
() => {
// Module: crate::references
// Provides: {"Declaration"}
// Dependencies: {}
# [doc = " Information about the declaration site of a searched item."] # [derive (Debug , Clone , UpmapFromRaFixture)] pub struct Declaration { # [doc = " Navigation information to jump to the declaration"] pub nav : NavigationTarget , # [doc = " Whether the declared item is mutable (relevant for variables)"] pub is_mut : bool , }
};
}
