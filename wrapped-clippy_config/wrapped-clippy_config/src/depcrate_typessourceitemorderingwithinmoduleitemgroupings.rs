// Generated macro for SourceItemOrderingWithinModuleItemGroupings (enum)
macro_rules! Depcrate_typesSourceItemOrderingWithinModuleItemGroupings {
() => {
// Module: crate::types
// Provides: {"SourceItemOrderingWithinModuleItemGroupings"}
// Dependencies: {}
# [doc = " Describes which specific groupings should have their items ordered"] # [doc = " alphabetically."] # [doc = ""] # [doc = " This is separate from defining and enforcing groupings. For example,"] # [doc = " defining enums are grouped before structs still allows for an enum B to be"] # [doc = " placed before an enum A. Only when enforcing ordering within the grouping,"] # [doc = " will it be checked if A is placed before B."] # [derive (Clone , Debug)] pub enum SourceItemOrderingWithinModuleItemGroupings { # [doc = " All groupings should have their items ordered."] All , # [doc = " None of the groupings should have their order checked."] None , # [doc = " Only the specified groupings should have their order checked."] Custom (Vec < String >) , }
};
}
