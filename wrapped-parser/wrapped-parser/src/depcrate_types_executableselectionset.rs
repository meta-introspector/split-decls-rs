// Generated macro for SelectionSet (struct)
macro_rules! Depcrate_types_executableSelectionSet {
() => {
// Module: crate::types::executable
// Provides: {"SelectionSet"}
// Dependencies: {}
# [doc = " A set of fields to be selected, for example `{ name age }`."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#SelectionSet)."] # [derive (Debug , Default , Clone , Serialize , Deserialize)] pub struct SelectionSet { # [doc = " The fields to be selected."] pub items : Vec < Positioned < Selection > > , }
};
}
