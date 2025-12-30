// Generated macro for FragmentSpread (struct)
macro_rules! Depcrate_types_executableFragmentSpread {
() => {
// Module: crate::types::executable
// Provides: {"FragmentSpread"}
// Dependencies: {}
# [doc = " A fragment selector, such as `... userFields`."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#FragmentSpread)."] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct FragmentSpread { # [doc = " The name of the fragment being selected."] pub fragment_name : Positioned < Name > , # [doc = " The directives in the fragment selector."] pub directives : Vec < Positioned < Directive > > , }
};
}
