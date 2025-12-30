// Generated macro for known_const_to_ast (function)
macro_rules! Depcrateknown_const_to_ast {
() => {
// Module: crate
// Provides: {"known_const_to_ast"}
// Dependencies: {}
pub fn known_const_to_ast < 'db > (konst : Const < 'db > , db : & 'db dyn HirDatabase , display_target : DisplayTarget ,) -> Option < ConstArg > { Some (make :: expr_const_value (konst . display (db , display_target) . to_string () . as_str ())) }
};
}
