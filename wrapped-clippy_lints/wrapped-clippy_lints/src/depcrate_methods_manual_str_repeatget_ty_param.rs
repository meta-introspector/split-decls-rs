// Generated macro for get_ty_param (function)
macro_rules! Depcrate_methods_manual_str_repeatget_ty_param {
() => {
// Module: crate::methods::manual_str_repeat
// Provides: {"get_ty_param"}
// Dependencies: {}
fn get_ty_param (ty : Ty < '_ >) -> Option < Ty < '_ > > { if let ty :: Adt (_ , subs) = ty . kind () { subs . types () . next () } else { None } }
};
}
