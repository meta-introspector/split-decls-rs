// Generated macro for attr_is (function)
macro_rules! Depcrate_utilsattr_is {
() => {
// Module: crate::utils
// Provides: {"attr_is"}
// Dependencies: {}
pub (crate) fn attr_is (attr : & Attribute , name : & str) -> bool { attr . path () . is_ident (& format_ident ! ("{}" , name)) }
};
}
