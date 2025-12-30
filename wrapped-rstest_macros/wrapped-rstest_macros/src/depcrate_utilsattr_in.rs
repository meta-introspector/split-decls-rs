// Generated macro for attr_in (function)
macro_rules! Depcrate_utilsattr_in {
() => {
// Module: crate::utils
// Provides: {"attr_in"}
// Dependencies: {}
pub (crate) fn attr_in (attr : & Attribute , names : & [& str]) -> bool { names . iter () . any (| name | attr . path () . is_ident (& format_ident ! ("{}" , name))) }
};
}
