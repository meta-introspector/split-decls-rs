// Generated macro for OptionOccurrence (struct)
macro_rules! Depcrate_option_if_let_elseOptionOccurrence {
() => {
// Module: crate::option_if_let_else
// Provides: {"OptionOccurrence"}
// Dependencies: {}
# [doc = " A struct containing information about occurrences of construct that this lint detects"] # [doc = ""] # [doc = " Such as:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " if let Some(..) = {..} else {..}"] # [doc = " ```"] # [doc = " or"] # [doc = " ```ignore"] # [doc = " match x {"] # [doc = "     Some(..) => {..},"] # [doc = "     None/_ => {..}"] # [doc = " }"] # [doc = " ```"] struct OptionOccurrence { option : String , method_sugg : String , some_expr : String , none_expr : String , }
};
}
