// Generated macro for impl_182 (impl)
macro_rules! Depcrate_validimpl_182 {
() => {
// Module: crate::valid
// Provides: {"impl_182"}
// Dependencies: {}
impl Enum < '_ > { fn validate (& self) -> Result < () > { check_non_field_attrs (& self . attrs) ? ; let has_display = self . has_display () ; for variant in & self . variants { variant . validate () ? ; if has_display && variant . attrs . display . is_none () && variant . attrs . transparent . is_none () && variant . attrs . fmt . is_none () { return Err (Error :: new_spanned (variant . original , "missing #[error(\"...\")] display attribute" ,)) ; } } Ok (()) } }
};
}
