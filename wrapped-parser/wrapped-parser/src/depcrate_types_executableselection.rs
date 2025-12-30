// Generated macro for Selection (enum)
macro_rules! Depcrate_types_executableSelection {
() => {
// Module: crate::types::executable
// Provides: {"Selection"}
// Dependencies: {}
# [doc = " A part of an object to be selected; a single field, a fragment spread or an"] # [doc = " inline fragment."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#Selection)."] # [derive (Debug , Clone , Serialize , Deserialize)] pub enum Selection { # [doc = " Select a single field, such as `name` or `weightKilos: weight(unit:"] # [doc = " KILOGRAMS)`."] Field (Positioned < Field >) , # [doc = " Select using a fragment."] FragmentSpread (Positioned < FragmentSpread >) , # [doc = " Select using an inline fragment."] InlineFragment (Positioned < InlineFragment >) , }
};
}
