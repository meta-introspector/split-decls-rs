// Generated macro for attr_ends_with (function)
macro_rules! Depcrate_utilsattr_ends_with {
() => {
// Module: crate::utils
// Provides: {"attr_ends_with"}
// Dependencies: {}
pub (crate) fn attr_ends_with (attr : & Attribute , segment : & syn :: PathSegment) -> bool { attr . path () . segments . iter () . last () == Some (segment) }
};
}
