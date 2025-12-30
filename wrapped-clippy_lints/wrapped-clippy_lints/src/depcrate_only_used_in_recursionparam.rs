// Generated macro for Param (struct)
macro_rules! Depcrate_only_used_in_recursionParam {
() => {
// Module: crate::only_used_in_recursion
// Provides: {"Param"}
// Dependencies: {}
struct Param { # [doc = " The function this is a parameter for."] fn_id : DefId , fn_kind : FnKind , # [doc = " The index of this parameter."] idx : usize , ident : Ident , # [doc = " Whether this parameter should be linted. Set by `Params::flag_for_linting`."] apply_lint : Cell < bool > , # [doc = " All the uses of this parameter."] uses : Vec < Usage > , }
};
}
