// Generated macro for FragmentDefinition (struct)
macro_rules! Depcrate_types_executableFragmentDefinition {
() => {
// Module: crate::types::executable
// Provides: {"FragmentDefinition"}
// Dependencies: {}
# [doc = " The definition of a fragment, such as `fragment userFields on User { name"] # [doc = " age }`."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#FragmentDefinition)."] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct FragmentDefinition { # [doc = " The type this fragment operates on."] pub type_condition : Positioned < TypeCondition > , # [doc = " Directives in the fragment."] pub directives : Vec < Positioned < Directive > > , # [doc = " The fragment's selection set."] pub selection_set : Positioned < SelectionSet > , }
};
}
