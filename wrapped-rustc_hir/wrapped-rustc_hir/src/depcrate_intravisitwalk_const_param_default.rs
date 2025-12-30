// Generated macro for walk_const_param_default (function)
macro_rules! Depcrate_intravisitwalk_const_param_default {
() => {
// Module: crate::intravisit
// Provides: {"walk_const_param_default"}
// Dependencies: {}
pub fn walk_const_param_default < 'v , V : Visitor < 'v > > (visitor : & mut V , ct : & 'v ConstArg < 'v > ,) -> V :: Result { visitor . visit_const_arg_unambig (ct) }
};
}
