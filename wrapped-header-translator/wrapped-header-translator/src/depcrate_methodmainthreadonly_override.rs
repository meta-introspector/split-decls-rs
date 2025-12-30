// Generated macro for mainthreadonly_override (function)
macro_rules! Depcrate_methodmainthreadonly_override {
() => {
// Module: crate::method
// Provides: {"mainthreadonly_override"}
// Dependencies: {}
fn mainthreadonly_override < 'a > (result_type : & Ty , argument_types : impl IntoIterator < Item = & 'a Ty > , parent_is_mainthreadonly : bool , is_class : bool , mainthreadonly_modifier : bool ,) -> bool { let mut result_type_requires_mainthreadmarker = result_type . requires_mainthreadmarker (parent_is_mainthreadonly) ; let mut any_argument_provides_mainthreadmarker = argument_types . into_iter () . any (| arg_ty | arg_ty . provides_mainthreadmarker (parent_is_mainthreadonly)) ; if parent_is_mainthreadonly { if is_class { result_type_requires_mainthreadmarker = true ; } else { any_argument_provides_mainthreadmarker = true ; } } if any_argument_provides_mainthreadmarker { false } else if result_type_requires_mainthreadmarker { true } else { mainthreadonly_modifier } }
};
}
