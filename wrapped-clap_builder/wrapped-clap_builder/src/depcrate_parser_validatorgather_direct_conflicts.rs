// Generated macro for gather_direct_conflicts (function)
macro_rules! Depcrate_parser_validatorgather_direct_conflicts {
() => {
// Module: crate::parser::validator
// Provides: {"gather_direct_conflicts"}
// Dependencies: {}
fn gather_direct_conflicts (cmd : & Command , id : & Id) -> Vec < Id > { let conf = if let Some (arg) = cmd . find (id) { gather_arg_direct_conflicts (cmd , arg) } else if let Some (group) = cmd . find_group (id) { gather_group_direct_conflicts (group) } else { debug_assert ! (false , "id={id:?} is unknown") ; Vec :: new () } ; debug ! ("Conflicts::gather_direct_conflicts id={id:?}, conflicts={conf:?}" ,) ; conf }
};
}
