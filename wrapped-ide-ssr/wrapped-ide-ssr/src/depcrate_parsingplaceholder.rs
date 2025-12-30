// Generated macro for Placeholder (struct)
macro_rules! Depcrate_parsingPlaceholder {
() => {
// Module: crate::parsing
// Provides: {"Placeholder"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq , Eq)] pub (crate) struct Placeholder { # [doc = " The name of this placeholder. e.g. for \"$a\", this would be \"a\""] pub (crate) ident : Var , # [doc = " A unique name used in place of this placeholder when we parse the pattern as Rust code."] stand_in_name : String , pub (crate) constraints : Vec < Constraint > , }
};
}
