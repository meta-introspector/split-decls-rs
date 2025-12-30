// Generated macro for get_arg_by_name (function)
macro_rules! Depcrate_macros_reflectget_arg_by_name {
() => {
// Module: crate::macros::reflect
// Provides: {"get_arg_by_name"}
// Dependencies: {}
# [doc = " Extracts an [`Argument`] from the provided [`Arguments`] by its [`Name`]."] # [must_use] pub const fn get_arg_by_name (args : Arguments , name : Name) -> Option < Argument > { let mut i = 0 ; while i < args . len () { let arg = args [i] ; if str_eq (arg . 0 , name) { return Some (arg) ; } i += 1 ; } None }
};
}
