// Generated macro for impl_111 (impl)
macro_rules! Depcrate_macro_optionsimpl_111 {
() => {
// Module: crate::macro_options
// Provides: {"impl_111"}
// Dependencies: {}
impl FieldLevelSetter { # [doc = " Get whether the setter should be emitted. The rules are the same as"] # [doc = " for `field_enabled`, except we only skip the setter if `setter(custom)` is present."] pub fn setter_enabled (& self) -> Option < bool > { if self . custom . is_some () { return self . custom . map (| x | ! x) ; } self . field_enabled () } # [doc = " Get whether or not this field-level setter indicates a setter and"] # [doc = " field should be emitted. The setter shorthand rules are that the"] # [doc = " presence of a `setter` with _any_ properties set forces the setter"] # [doc = " to be emitted."] pub fn field_enabled (& self) -> Option < bool > { if self . skip . is_some () { return self . skip . map (| x | ! x) ; } if self . prefix . is_some () || self . name . is_some () || self . into . is_some () || self . strip_option . is_some () || self . each . is_some () { return Some (true) ; } None } }
};
}
