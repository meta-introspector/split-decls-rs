// Generated macro for attr_starts_with (function)
macro_rules! Depcrate_utilsattr_starts_with {
() => {
// Module: crate::utils
// Provides: {"attr_starts_with"}
// Dependencies: {}
pub (crate) fn attr_starts_with (attr : & Attribute , segment : & syn :: PathSegment) -> bool { attr . path () . segments . iter () . next () == Some (segment) }
};
}
